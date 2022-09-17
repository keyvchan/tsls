use database::{GlobalStateSnapshot, SourceDatabase};
use log::debug;
use lsp_types::notification::{Notification, PublishDiagnostics};

pub fn publish_diagnostics(
    uri: lsp_types::Url,
    global_state: GlobalStateSnapshot,
) -> lsp_server::Notification {
    // accuire the lock
    let diagnostics = global_state.diagnostics.lock();
    // get the diagnostics for the file
    let diagnostics = diagnostics.get(&uri).unwrap();

    debug!("publish_diagnostics: {:?}", diagnostics);

    let params = lsp_types::PublishDiagnosticsParams {
        uri: uri.clone(),
        diagnostics: diagnostics.to_vec(),
        version: Some(global_state.db.source(uri).version),
    };

    let result = serde_json::to_value(&params).unwrap();
    lsp_server::Notification {
        method: PublishDiagnostics::METHOD.to_string(),
        params: result,
    }
}
