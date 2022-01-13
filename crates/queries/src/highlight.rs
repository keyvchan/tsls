use lsp_types::{CompletionItemKind, TextDocumentItem};
use std::collections::HashMap;
use tree_sitter::{Language, Parser, Range, Tree};

use crate::capture_by_query_source;
use crate::utils::{get_query_source, get_smallest_scope_id_by_node, Symbol};

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
        let completion_item_kind = get_completion_kind(capture_name);
        let belongs_to = scopes[0..smallest_scope_id].to_owned();

        if visited_names.contains(&(smallest_scope_id, variable_name.to_string())) {
            // insert completion_kind
            let symbol = result
                .get_mut(&(smallest_scope_id, variable_name.to_string()))
                .unwrap();
            symbol.completion_kind.push(completion_item_kind);
        } else {
            // insert into visited_names
            visited_names.push((smallest_scope_id, variable_name.to_string()));
            // insert a name in the result
            let symbol = Symbol {
                name: variable_name.to_string(),
                completion_kind: vec![completion_item_kind],
                location: node.range(),
                belongs_to,
            };

            result.insert((smallest_scope_id, variable_name.to_string()), symbol);
        }
    }

    for ((id, name), value) in result {
        let result = identifiers.get_mut(&id).unwrap();
        result.push(value);
    }
}

fn get_completion_kind(capture_name: String) -> CompletionItemKind {
    match capture_name.as_str() {
        "variable" => CompletionItemKind::VARIABLE,
        "function" | "function.macro" => CompletionItemKind::FUNCTION,
        "type" => CompletionItemKind::TYPE_PARAMETER,
        "label" => CompletionItemKind::TEXT,
        "module" => CompletionItemKind::MODULE,
        "keyword" | "repeat" | "keyword.operator" | "keyword.return" => CompletionItemKind::KEYWORD,
        "struct" => CompletionItemKind::STRUCT,
        "enum" => CompletionItemKind::ENUM,
        "number" | "character" | "boolean" => CompletionItemKind::VALUE,
        "interface" => CompletionItemKind::INTERFACE,
        "constant" | "constant.builtin" => CompletionItemKind::CONSTANT,
        "string" | "string.escape" => CompletionItemKind::TEXT,
        "include" => CompletionItemKind::MODULE,
        "parameter" => CompletionItemKind::VARIABLE,
        "property" => CompletionItemKind::PROPERTY,
        "method" => CompletionItemKind::METHOD,
        "constructor" => CompletionItemKind::CONSTRUCTOR,
        "field" => CompletionItemKind::FIELD,
        "file" => CompletionItemKind::FILE,
        "package" => CompletionItemKind::MODULE,
        "namespace" => CompletionItemKind::MODULE,
        "class" => CompletionItemKind::CLASS,
        "enum_member" => CompletionItemKind::ENUM_MEMBER,
        "getter" => CompletionItemKind::PROPERTY,
        "setter" => CompletionItemKind::PROPERTY,
        "operator"
        | "punctuation"
        | "punctuation.bracket"
        | "punctuation.delimiter"
        | "punctuation.special"
        | "conditional" => CompletionItemKind::OPERATOR,
        _ => CompletionItemKind::TEXT,
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
        for (capture_name, node) in capture_by_query_source(
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

    keywords
}
