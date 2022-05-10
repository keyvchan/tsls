use helper::convert::ts_range_to_lsp_range;
use lsp_server::{ErrorCode::ParseError, RequestId, Response};
use lsp_types::{DocumentSymbol, DocumentSymbolParams};

use crate::global_state::GlobalState;

pub fn document_symbol(
    id: RequestId,
    params: DocumentSymbolParams,
    state: GlobalState,
) -> Response {
    let uri = params.text_document.uri;

    let properties = if let Some(properties) = state.sources.get(&uri) {
        properties
    } else {
        return Response::new_err(
            id,
            ParseError as i32,
            "No properties found for this document".to_string(),
        );
    };

    let mut document_symbols: Vec<DocumentSymbol> = Vec::new();

    let identifiers = &properties.identifiers;

    for symbols in identifiers.values() {
        #[allow(deprecated)]
        for symbol in symbols {
            // debug!("symbol: {:?}, {:?}", symbol.name, symbol.belongs_to());
            document_symbols.push(DocumentSymbol {
                name: symbol.name.clone(),
                detail: None,
                kind: *symbol.symbol_kind.last().unwrap(),
                tags: None,

                deprecated: None,

                // TODO: Return maxium scope
                // The whole scope of this struct
                range: ts_range_to_lsp_range(symbol.belongs_to().last().unwrap()),

                // struct name
                selection_range: ts_range_to_lsp_range(&symbol.location),

                children: None,
            });
        }
    }

    // debug!("Document symbols: {:?}", document_symbols);

    // Only can go to current files
    let result = serde_json::to_value(&document_symbols).unwrap();
    lsp_server::Response {
        id,
        result: Some(result),
        error: None,
    }
}
