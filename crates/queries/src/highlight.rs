use std::collections::HashMap;

use helper::types::Symbol;
use log::{error, info};
use lsp_types::{CompletionItemKind, SymbolKind, TextDocumentItem};
use tree_sitter::{Language, Parser, Range, Tree};

use crate::{
    capture_by_query_source,
    utils::{get_query_source, get_smallest_scope_id_by_node},
};

/// Update kind of the item in definitions.
pub fn update_identifiers_kind(
    identifiers: &mut HashMap<usize, Vec<Symbol>>,
    scopes: &[Range],
    source_code: &TextDocumentItem,
    tree: &Tree,
) {
    let source = get_query_source(source_code.language_id.as_str(), "highlights").unwrap();
    let captures = capture_by_query_source(source_code.text.clone(), tree.root_node(), &source);

    let mut visited_names: Vec<(usize, String)> = vec![];
    let mut result = HashMap::<(usize, String), Symbol>::new();
    for (capture_name, node) in captures {
        let smallest_scope_id = get_smallest_scope_id_by_node(&node, scopes);
        let variable_name = node.utf8_text(source_code.text.as_bytes()).unwrap();
        let (completion_item_kind, symbol_kind) = get_kind(capture_name);

        // TODO: Better way to handle this.
        let mut belongs_to_scopes = scopes[0..smallest_scope_id].to_owned();
        if smallest_scope_id == 0 {
            // if variable not in any scope, we assume it in the maxium scope
            if scopes.is_empty() {
                belongs_to_scopes = vec![]
            } else {
                belongs_to_scopes = vec![scopes[0]];
            }
        }

        if visited_names.contains(&(smallest_scope_id, variable_name.to_string())) {
            // insert completion_kind
            // if it already exist, update it
            let symbol = result
                .get_mut(&(smallest_scope_id, variable_name.to_string()))
                .unwrap();
            symbol.completion_kind.push(completion_item_kind);
            symbol.symbol_kind.push(symbol_kind);
        } else {
            // insert into visited_names
            visited_names.push((smallest_scope_id, variable_name.to_string()));
            // insert a name in the result
            let symbol = Symbol {
                name: variable_name.to_string(),
                completion_kind: vec![completion_item_kind],
                symbol_kind: vec![symbol_kind],
                location: node.range(),
                children: None,
                belongs_to_scopes,
            };

            result.insert((smallest_scope_id, variable_name.to_string()), symbol);
        }
    }

    let mut empty_vec = vec![];

    for ((id, _name), value) in result {
        let result = match identifiers.get_mut(&id) {
            Some(result) => result,
            None => &mut empty_vec,
        };
        result.push(value);
    }
}

/// Get completion_kind and symbol_kind
fn get_kind(capture_name: String) -> (CompletionItemKind, SymbolKind) {
    match capture_name.as_str() {
        "variable" => (CompletionItemKind::VARIABLE, SymbolKind::VARIABLE),
        "function" | "function.macro" => (CompletionItemKind::FUNCTION, SymbolKind::FUNCTION),
        "type" => (
            CompletionItemKind::TYPE_PARAMETER,
            SymbolKind::TYPE_PARAMETER,
        ),
        "label" => (CompletionItemKind::TEXT, SymbolKind::STRING),
        "module" => (CompletionItemKind::MODULE, SymbolKind::MODULE),
        "keyword" | "repeat" | "keyword.operator" | "keyword.return" => {
            (CompletionItemKind::KEYWORD, SymbolKind::KEY)
        }
        "struct" => (CompletionItemKind::STRUCT, SymbolKind::STRUCT),
        "enum" => (CompletionItemKind::ENUM, SymbolKind::ENUM),
        "number" | "character" | "boolean" => (CompletionItemKind::VALUE, SymbolKind::NUMBER),
        "interface" => (CompletionItemKind::INTERFACE, SymbolKind::INTERFACE),
        "constant" | "constant.builtin" => (CompletionItemKind::CONSTANT, SymbolKind::CONSTANT),
        "string" | "string.escape" => (CompletionItemKind::TEXT, SymbolKind::STRING),
        "include" => (CompletionItemKind::MODULE, SymbolKind::MODULE),
        "parameter" => (CompletionItemKind::VARIABLE, SymbolKind::VARIABLE),
        "property" => (CompletionItemKind::PROPERTY, SymbolKind::PROPERTY),
        "method" => (CompletionItemKind::METHOD, SymbolKind::METHOD),
        "constructor" => (CompletionItemKind::CONSTRUCTOR, SymbolKind::CONSTRUCTOR),
        "field" => (CompletionItemKind::FIELD, SymbolKind::FIELD),
        "file" => (CompletionItemKind::FILE, SymbolKind::FILE),
        "package" => (CompletionItemKind::MODULE, SymbolKind::MODULE),
        "namespace" => (CompletionItemKind::MODULE, SymbolKind::MODULE),
        "class" => (CompletionItemKind::CLASS, SymbolKind::CLASS),
        "enum_member" => (CompletionItemKind::ENUM_MEMBER, SymbolKind::ENUM_MEMBER),
        "getter" => (CompletionItemKind::PROPERTY, SymbolKind::PROPERTY),
        "setter" => (CompletionItemKind::PROPERTY, SymbolKind::PROPERTY),
        "operator"
        | "punctuation"
        | "punctuation.bracket"
        | "punctuation.delimiter"
        | "punctuation.special"
        | "conditional" => (CompletionItemKind::OPERATOR, SymbolKind::OPERATOR),
        _ => (CompletionItemKind::TEXT, SymbolKind::STRING),
    }
}

// Return all keywords for language
// TODO: Nasty hack, should be done in a better way
pub fn build_keywords_cache(language_id: String) -> Vec<String> {
    let source = get_query_source(language_id.as_str(), "highlights").unwrap();

    let mut parser = Parser::new();

    // Default language is C
    let language: Language = tree_sitter_query::language();
    parser.set_language(language).unwrap();

    let tree = parser.parse(source.clone(), None).unwrap();

    let mut lists: Vec<String> = vec![];

    for (capture_name, node) in capture_by_query_source(
        source.clone(),
        tree.root_node(),
        r#"
        (list
            (
                capture
                    name: (identifier) @capture
            ) 
        )@list

        (
            capture
                name: (identifier) @capture
        ) 

        "#,
    ) {
        let node_content = node.utf8_text(source.as_bytes()).unwrap();
        match capture_name.as_str() {
            "list" => {
                lists.push(node_content.to_string());
            }
            "capture" => match node_content {
                "keyword" | "repeat" => {}
                _ => {
                    lists.pop();
                }
            },
            _ => {}
        }
    }

    let mut keywords: Vec<String> = vec![];

    for item in lists {
        let new_tree = parser.parse(item.clone(), None).unwrap();
        for (_capture_name, node) in capture_by_query_source(
            item.clone(),
            new_tree.root_node(),
            r#"
                (
                    list
                        (
                            anonymous_node
                                name: (identifier) @keyword
                        )
                )
            "#,
        ) {
            let node_content = node
                .utf8_text(item.as_bytes())
                .unwrap()
                .trim_start_matches('"')
                .trim_end_matches('"');
            keywords.push(node_content.to_string());
        }
    }
    error!("{:?}", keywords);

    keywords
}
