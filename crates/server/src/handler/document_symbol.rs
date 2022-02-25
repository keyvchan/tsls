use helper::convert::ts_range_to_lsp_range;
use log::debug;
use lsp_server::RequestId;
use lsp_server::Response;
use lsp_types::error_codes::REQUEST_CANCELLED;
use lsp_types::DocumentSymbol;
use lsp_types::DocumentSymbolParams;

use crate::global_state::GlobalState;

pub fn document_symbol(
    id: RequestId,
    params: DocumentSymbolParams,
    state: GlobalState,
) -> Response {
    debug!("document_symbol");
    let uri = params.text_document.uri;

    let properties = if let Some(properties) = state.sources.get(&uri) {
        properties
    } else {
        return Response::new_err(
            id,
            REQUEST_CANCELLED as i32,
            "No properties found for this document".to_string(),
        );
    };

    let mut document_symbols: Vec<DocumentSymbol> = Vec::new();

    let identifiers = &properties.identifiers;

    for (scope_id, symbols) in identifiers {
        #[allow(deprecated)]
        for symbol in symbols {
            debug!("symbol: {:?}, {:?}", symbol.name, symbol.belongs_to());
            document_symbols.push(DocumentSymbol {
                name: symbol.name.clone(),
                detail: None,
                kind: *symbol.symbol_kind.last().unwrap(),
                tags: None,

                deprecated: None,

                // TODO: Return maxium scope
                range: ts_range_to_lsp_range(symbol.belongs_to().last().unwrap()),
                selection_range: ts_range_to_lsp_range(&symbol.location),

                children: None,
            });
        }
    }

    debug!("Document symbols: {:?}", document_symbols);

    // Only can go to current files
    let result = serde_json::to_value(&document_symbols).unwrap();
    lsp_server::Response {
        id,
        result: Some(result),
        error: None,
    }
}
