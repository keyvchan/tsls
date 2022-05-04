use log::{debug, error};

use crate::global_state::GlobalState;

pub fn did_save(params: lsp_types::DidSaveTextDocumentParams, global_state: &mut GlobalState) {
    debug!("Received a DidSaveTextDocumentParams: {:?}", params);

    let uri = &params.text_document.uri;

    global_state.update_cache(uri);

    // update diagnostics
    match global_state.update_diagnostics(uri) {
        Ok(()) => (),
        Err(e) => {
            error!("{}", e);
        }
    }
}
