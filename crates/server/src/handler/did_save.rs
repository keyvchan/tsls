use database::GlobalState;
use log::debug;

pub fn did_save(params: lsp_types::DidSaveTextDocumentParams, global_state: &mut GlobalState) {
    debug!("Received a DidSaveTextDocumentParams: {:?}", params);
    // TODO: update diagnostics
}
