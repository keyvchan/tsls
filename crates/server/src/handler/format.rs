use lsp_server::{RequestId, Response};
use lsp_types::DocumentFormattingParams;

use crate::global_state::GlobalState;

pub fn format(
    id: RequestId,
    params: DocumentFormattingParams,
    global_state: GlobalState,
) -> Response {
    global_state.get_source_code(&params.text_document.uri);
    let result = Some(vec![0]);
    let result = serde_json::to_value(&result).unwrap();
    lsp_server::Response {
        id,
        result: Some(result),
        error: None,
    }
}
