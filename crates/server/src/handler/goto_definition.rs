use log::{debug, error};

use crate::global_state::GlobalState;

pub fn goto_definition(
    id: lsp_server::RequestId,
    params: lsp_types::GotoDefinitionParams,
    global_state: GlobalState,
) -> lsp_server::Response {
    debug!("got gotoDefinition request #{}: {:?}", id, params);
    let properties = global_state
        .sources
        .get(&params.text_document_position_params.text_document.uri)
        .unwrap();
    let tree = &properties.ast;
    let scopes = &properties.ordered_scopes;
    let source_code = &properties.source_code;

    let node_position = params.text_document_position_params.position;
    let point = tree_sitter::Point::new(
        node_position.line.try_into().unwrap(),
        node_position.character.try_into().unwrap(),
    );

    // Find that node
    let root_node = tree.root_node();
    let tree_cursor = root_node.walk();
    let ranges: Vec<lsp_types::Range> = match tree_cursor
        .node()
        .named_descendant_for_point_range(point, point)
    {
        Some(node) => {
            // check current scope

            let loopup_table = &properties.definitions_lookup_map;
            let smallest_scope_id = queries::utils::get_smallest_scope_id_by_node(&node, scopes);

            let variable_name = node.utf8_text(source_code).unwrap().to_owned();

            let key = format!("{}:{}", variable_name, smallest_scope_id);
            let definitions = match loopup_table.get(&key) {
                Some(definitions) => definitions,
                None => {
                    error!("could not find definition for {}", key);
                    return lsp_server::Response::new_err(
                        id,
                        lsp_types::error_codes::REQUEST_CANCELLED as i32,
                        "could not find definition for this variable".to_string(),
                    );
                }
            };

            let range = &definitions[0];
            let start = lsp_types::Position {
                line: range.location.start_point.row.try_into().unwrap(),
                character: range.location.start_point.column.try_into().unwrap(),
            };
            let end = lsp_types::Position {
                line: range.location.end_point.row.try_into().unwrap(),
                character: range.location.end_point.column.try_into().unwrap(),
            };
            let range = lsp_types::Range { start, end };

            vec![range]
        }
        None => {
            error!("no node found");
            vec![]
        }
    };

    // Only can go to current files
    let location = vec![lsp_types::Location {
        uri: params.text_document_position_params.text_document.uri,
        range: lsp_types::Range {
            start: lsp_types::Position {
                line: ranges[0].start.line,
                character: ranges[0].start.character,
            },
            end: lsp_types::Position {
                line: ranges[0].end.line,
                character: ranges[0].end.character,
            },
        },
    }];

    let result = Some(lsp_types::GotoDefinitionResponse::Array(location));
    let result = serde_json::to_value(&result).unwrap();
    lsp_server::Response::new_ok(id, result)
}
