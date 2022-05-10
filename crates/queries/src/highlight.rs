use std::{collections::HashMap, str::from_utf8};

use helper::types::Symbol;
use log::debug;
use lsp_types::{CompletionItemKind, SymbolKind};
use tree_sitter::{Language, Parser, Range, Tree};

use crate::{
    capture_by_query_source, match_by_query_source,
    utils::{get_query_source, get_smallest_scope_id_by_node},
};

/// Update kind of the item in definitions.
pub fn update_identifiers_kind(
    identifiers: &mut HashMap<usize, Vec<Symbol>>,
    scopes: &[Range],
    source_code: &Vec<u8>,
    tree: &Tree,
    language_id: &str,
) {
    let source = get_query_source(language_id, "highlights").unwrap();
    let captures = capture_by_query_source(
        source_code,
        tree.root_node(),
        from_utf8(source.as_bytes()).unwrap(),
    );

    let mut visited_names: Vec<(usize, String)> = vec![];
    let mut result = HashMap::<(usize, String), Symbol>::new();
    for (capture_name, node) in captures {
        let smallest_scope_id = get_smallest_scope_id_by_node(&node, scopes);
        let variable_name = node.utf8_text(source_code).unwrap();
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

    let tree = parser.parse(&source, None).unwrap();
    let mut keywords = vec![];

    let keywords_capture: Vec<&str> = vec![
        "keyword",
        "keyword.operator",
        "keyword.return",
        "keyword.function",
        "conditional",
    ];

    for matches in match_by_query_source(
        &source.as_bytes().to_vec(),
        tree.root_node(),
        r#"
            (anonymous_node
              name: (identifier) @node_name
              (capture
                name: (identifier) @capture_name
              )
            )
            (list
              (anonymous_node
                name: (identifier)
              )+ @node_name
              (capture
                name: (identifier) @capture_name
              )
            )
    "#,
    ) {
        // the last one of matches should be the name
        if matches
            .last()
            .is_some_and(|m| keywords_capture.contains(&m.1.utf8_text(source.as_bytes()).unwrap()))
        {
            // add to keywords cache, exclude len == 1
            for (_, m) in matches.iter().take(matches.len() - 1) {
                let mut node_content = m.utf8_text(source.as_bytes()).unwrap_or("");
                node_content = node_content.trim_start_matches('"').trim_end_matches('"');
                if node_content.len() > 1 {
                    keywords.push(node_content.to_string());
                }
            }
        }
    }

    debug!("{:?}", keywords);

    keywords
}
