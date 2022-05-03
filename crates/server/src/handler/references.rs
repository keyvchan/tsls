use helper::tree_walker::get_named_node_by_position;
use lsp_server::{ErrorCode::ParseError, RequestId, Response};
use lsp_types::ReferenceParams;
use queries::utils::get_smallest_scope_id_by_node;

use crate::global_state::GlobalState;

pub fn references(id: RequestId, params: ReferenceParams, state: GlobalState) -> Response {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let tree = if let Some(tree) = state.get_tree(&uri) {
        tree
    } else {
        return Response::new_err(
            id,
            ParseError as i32,
            "No tree found for this document".to_string(),
        );
    };
    let node = if let Some(node) = get_named_node_by_position(tree, position) {
        node
    } else {
        return Response::new_err(
            id,
            ParseError as i32,
            "No node found for this position".to_string(),
        );
    };

    let properties = if let Some(properties) = state.sources.get(&uri) {
        properties
    } else {
        return Response::new_err(
            id,
            ParseError as i32,
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
            ParseError as i32,
            "No variable name found for this node".to_string(),
        );
    };

    let key = format!("{}:{}", variable_name, smallest_scope_id);
    let definitions = if let Some(definitions) = loopup_table.get(&key) {
        definitions
    } else {
        return Response::new_err(
            id,
            ParseError as i32,
            "No definitions found for this variable".to_string(),
        );
    };

    let mut locations = Vec::new();

    // Only can go to current files
    for symbol in definitions.iter() {
        let location = lsp_types::Location {
            uri: uri.clone(),
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
        locations.push(location);
    }

    let result = Some(locations);
    let result = serde_json::to_value(&result).unwrap();
    lsp_server::Response {
        id,
        result: Some(result),
        error: None,
    }
}
