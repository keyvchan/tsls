use std::collections::HashMap;

use helper::{tree_walker::get_named_node_by_position, types::Symbol};
use lsp_server::{RequestId, Response};
use lsp_types::{error_codes::REQUEST_CANCELLED, RenameParams, TextEdit, Url, WorkspaceEdit};
use queries::utils::get_smallest_scope_id_by_node;

use crate::global_state::GlobalState;

/// Setp:
/// 1. Find the smallest scope id of the node
/// 2. Find all nodes in the smallest scope id
/// 3. Rename the node
/// 4. Send the response
pub fn rename(id: RequestId, params: RenameParams, state: GlobalState) -> Response {
    // uri, position, tree, node
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let tree = if let Some(tree) = state.get_tree(&uri) {
        tree
    } else {
        return Response::new_err(
            id,
            REQUEST_CANCELLED as i32,
            "No tree found for this document".to_string(),
        );
    };
    let node = if let Some(node) = get_named_node_by_position(tree, position) {
        node
    } else {
        return Response::new_err(
            id,
            REQUEST_CANCELLED as i32,
            "No node found for this position".to_string(),
        );
    };

    // properties
    let properties = if let Some(properties) = state.sources.get(&uri) {
        properties
    } else {
        return Response::new_err(
            id,
            REQUEST_CANCELLED as i32,
            "No properties found for this document".to_string(),
        );
    };

    let loopup_table = &properties.definitions_lookup_map;
    let smallest_scope_id = get_smallest_scope_id_by_node(&node, &properties.ordered_scopes);

    let variable_name = if let Ok(variable_name) = node.utf8_text(&properties.source_code) {
        variable_name
    } else {
        return Response::new_err(
            id,
            REQUEST_CANCELLED as i32,
            "No variable name found for this node".to_string(),
        );
    };

    // construct key in the lookup table
    let key = format!("{}:{}", variable_name, smallest_scope_id);
    let definitions = if let Some(definitions) = loopup_table.get(&key) {
        definitions
    } else {
        return Response::new_err(
            id,
            REQUEST_CANCELLED as i32,
            "No definitions found for this variable".to_string(),
        );
    };

    // Found all the locations, construct the response

    let result = Some(get_response(uri, definitions, params.new_name));

    let result = serde_json::to_value(&result).unwrap();
    lsp_server::Response {
        id,
        result: Some(result),
        error: None,
    }
}

fn get_response(url: Url, definitions: &[Symbol], new_text: String) -> WorkspaceEdit {
    let mut text_edits = Vec::new();

    for symbol in definitions.iter() {
        let text_edit = lsp_types::TextEdit {
            new_text: new_text.clone(),
            range: lsp_types::Range {
                start: lsp_types::Position {
                    line: symbol.location.start_point.row as u32,
                    character: symbol.location.start_point.column as u32,
                },
                end: lsp_types::Position {
                    line: symbol.location.end_point.row as u32,
                    character: symbol.location.end_point.column as u32,
                },
            },
        };
        text_edits.push(text_edit);
    }
    let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();
    changes.insert(url, text_edits);

    WorkspaceEdit::new(changes)
}
