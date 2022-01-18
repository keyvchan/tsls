use crate::global_state::GlobalState;

pub fn references(
    id: lsp_server::RequestId,
    params: lsp_types::ReferenceParams,
    state: GlobalState,
) -> lsp_server::Response {
    lsp_server::Response::new_err(
        id,
        lsp_types::error_codes::REQUEST_CANCELLED as i32,
        "Not implemented".to_string(),
    )
}
