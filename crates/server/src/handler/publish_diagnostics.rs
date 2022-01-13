use crate::global_state::GlobalState;
use log::error;
use lsp_types::notification::Notification;

pub fn publish_diagnostics(
    uri: lsp_types::Url,
    global_state: GlobalState,
) -> lsp_server::Notification {
    let diagnostics = global_state.get_diagnostics(&uri).unwrap_or_default();

    error!("publish_diagnostics: {:?}", diagnostics);

    let params = lsp_types::PublishDiagnosticsParams {
        uri: uri.clone(),
        diagnostics: diagnostics.to_vec(),
        version: global_state.get_version(&uri),
    };

    let result = serde_json::to_value(&params).unwrap();
    lsp_server::Notification {
        method: lsp_types::notification::PublishDiagnostics::METHOD.to_string(),
        params: result,
    }
}
