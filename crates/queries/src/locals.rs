use std::collections::HashMap;

use helper::types::Symbol;
use log::error;
use lsp_types::{CompletionItemKind, SymbolKind};
use tree_sitter::Node;

use crate::{
    capture_by_query_source, match_by_query_source,
    utils::{get_query_source, get_smallest_scope_id_by_node},
};

// TODO: Hard-coded for now.
pub const REFERENCE: &str = "reference";
pub const DIFINITION_VAR: &str = "definition.var";
pub const DIFINITION_FUNCTION: &str = "definition.function";
pub const SCOPE: &str = "scope";

/// build scopes from biggest to smallest
fn build_scopes(
    source_code: &lsp_types::TextDocumentItem,
    node: Node,
) -> (Vec<tree_sitter::Range>, HashMap<usize, Vec<Symbol>>) {
    let query_source = {
        if let Some(x) = get_query_source(source_code.language_id.as_str(), "locals") {
            x
        } else {
            "".to_string()
        }
    };

    let result = match_by_query_source(source_code, node, query_source.as_str());

    // A scope contains a node
    let mut identifiers: HashMap<usize, Vec<Symbol>> = HashMap::new();

    let mut scopes: Vec<tree_sitter::Range> = Vec::new();

    let mut scope_id = 0;
    for item in &result {
        for (variable_type, node) in item {
            if variable_type.as_str() == SCOPE {
                scopes.push(node.range());
                identifiers.insert(scope_id, vec![]);
                scope_id += 1;
            }
        }
    }

    (scopes, identifiers)
}

fn build_definitions_and_identifiers(
    source_code: &lsp_types::TextDocumentItem,
    node: Node,
    scopes: &[tree_sitter::Range],
) -> HashMap<String, Vec<Symbol>> {
    error!("build_definitions_and_identifiers: {:?}", source_code);
    let query_source = {
        if let Some(x) = get_query_source(source_code.language_id.as_str(), "locals") {
            x
        } else {
            "".to_string()
        }
    };
    let result = capture_by_query_source(
        source_code.text.clone(),
        node.to_owned(),
        query_source.as_str(),
    );
    let mut definitions: HashMap<String, Vec<Symbol>> = HashMap::new();

    // use name + smallest_scope_id as key
    for (variable_type, node) in result {
        // for (variable_type, node) in item {

        let variable_name = node.utf8_text(source_code.text.as_bytes()).unwrap();
        match variable_type.as_str() {
            DIFINITION_VAR | DIFINITION_FUNCTION => {
                let smallest_scope_id = get_smallest_scope_id_by_node(&node, scopes);
                let key = format!("{}:{}", variable_name, smallest_scope_id);

                // check if the key already exists
                if definitions.contains_key(&key) {
                    // we do nothing
                    continue;
                };
                definitions.insert(key, vec![]);
            }
            REFERENCE => {
                let smallest_scope_id = get_smallest_scope_id_by_node(&node, scopes);

                let key = format!("{}:{}", variable_name, smallest_scope_id);

                let belongs_to_scopes = scopes[0..smallest_scope_id].to_owned();
                let symbol = Symbol {
                    name: variable_name.to_owned(),
                    completion_kind: vec![CompletionItemKind::TEXT],
                    symbol_kind: vec![SymbolKind::STRING],
                    location: node.range(),
                    children: None,
                    belongs_to_scopes,
                };

                let vector = match definitions.get_mut(&key) {
                    Some(v) => v,
                    None => {
                        // we don't have that variable, just ignore it
                        continue;
                    }
                };
                vector.push(symbol.clone());
            }
            _ => {
                // TODO: check children
                // ignore
            }
        }
    }

    // TODO: query struct/class fields, add it to children
    let query_source = {
        if let Some(x) = get_query_source(source_code.language_id.as_str(), "children") {
            x
        } else {
            "".to_string()
        }
    };
    let result = capture_by_query_source(
        source_code.text.clone(),
        node.to_owned(),
        query_source.as_str(),
    );

    for (capture, node) in result {
        error!("{:#?}, {:#?}", capture, &node);
    }

    definitions
}

/// Build the definition map and scope map for the first time
/// Called when didOpen
pub fn build_definitions_and_scopes(
    source_code: &lsp_types::TextDocumentItem,
    root_node: &tree_sitter::Node,
) -> (
    HashMap<String, Vec<Symbol>>,
    Vec<tree_sitter::Range>,
    HashMap<usize, Vec<Symbol>>,
) {
    let (scopes, identifiers) = build_scopes(source_code, root_node.to_owned());
    let definitions = build_definitions_and_identifiers(source_code, root_node.to_owned(), &scopes);

    (definitions, scopes, identifiers)
}
