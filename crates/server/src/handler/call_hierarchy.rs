use log::debug;
use lsp_server::{RequestId, Response};
use lsp_types::CallHierarchyPrepareParams;

use crate::global_state::GlobalState;

pub fn call_hierarchy(
    id: RequestId,
    params: CallHierarchyPrepareParams,
    global_state: GlobalState,
) -> Response {
    debug!("got call hierarchy prepear request #{}: {:#?}", id, params);
    unimplemented!();

    // let properties = global_state
    //     .sources
    //     .get(&params.text_document_position_params.text_document.uri)
    //     .unwrap();
    // let tree = &properties.ast;
    // let scopes = &properties.ordered_scopes;
    // let source_code = &properties.source_code;

    // let node_position = params.text_document_position_params.position;
    // let point = tree_sitter::Point::new(
    //     node_position.line.try_into().unwrap(),
    //     node_position.character.try_into().unwrap(),
    // );

    // // Find that node
    // let root_node = tree.root_node();
    // let tree_cursor = root_node.walk();

    // Only can go to current files
    // let call_hierarchy_items: Vec<CallHierarchyItem> = vec![];

    // let result = serde_json::to_value(&call_hierarchy_items).unwrap();
    // lsp_server::Response {
    //     id,
    //     result: Some(result),
    //     error: None,
    // }
}
