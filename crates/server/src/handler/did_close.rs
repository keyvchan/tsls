use database::GlobalState;

pub fn did_close(params: lsp_types::DidCloseTextDocumentParams, global_state: &mut GlobalState) {
    global_state.clear(&params.text_document.uri);
}
