use log::{debug, error};
use queries::{highlight::update_identifiers_kind, locals::build_definitions_and_scopes};

use crate::global_state::GlobalState;

pub fn did_save(params: lsp_types::DidSaveTextDocumentParams, global_state: &mut GlobalState) {
    debug!("Received a DidSaveTextDocumentParams: {:?}", params);

    let mut properties = global_state
        .sources
        .get_mut(&params.text_document.uri)
        .unwrap();

    // we rebuild the identifier
    let (definitions_lookup_map, ordered_scopes, mut identifiers) = build_definitions_and_scopes(
        &properties.source_code.to_vec(),
        &properties.ast.root_node(),
        &properties.language_id,
    );

    update_identifiers_kind(
        &mut identifiers,
        &ordered_scopes,
        &properties.source_code,
        &properties.ast,
        &properties.language_id,
    );
    // Save it to the global state
    properties.identifiers = identifiers;
    properties.definitions_lookup_map = definitions_lookup_map;
    properties.ordered_scopes = ordered_scopes;

    // update diagnostics
    match global_state.update_diagnostics(&params.text_document.uri) {
        Ok(()) => (),
        Err(e) => {
            error!("{}", e);
        }
    }
}
