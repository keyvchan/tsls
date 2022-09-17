use database::GlobalState;
use lsp_server::{ErrorCode::ParseError, RequestId, Response};
use lsp_types::{error_codes::REQUEST_CANCELLED, DocumentFormattingParams, TextEdit};
use queries::indents::text_edits;

pub fn format(
    id: RequestId,
    params: DocumentFormattingParams,
    global_state: GlobalState,
) -> Response {
    let source_code = match global_state.get_source_code(&params.text_document.uri) {
        Some(source_code) => source_code,
        None => {
            return Response::new_err(
                id,
                REQUEST_CANCELLED as i32,
                "Document not found".to_string(),
            )
        }
    };

    let language = match global_state.get_language_id(&params.text_document.uri) {
        Some(language) => language,
        None => {
            return Response::new_err(
                id,
                REQUEST_CANCELLED as i32,
                "Language not found".to_string(),
            )
        }
    };

    let old_tree = match global_state.get_tree(&params.text_document.uri) {
        Some(tree) => tree,
        None => {
            return Response::new_err(id, REQUEST_CANCELLED as i32, "Tree not found".to_string())
        }
    };

    // get text edit
    let text_edits = match text_edits(source_code, &language, old_tree) {
        Ok(text_edits) => text_edits,
        Err(e) => return Response::new_err(id, ParseError as i32, e),
    };

    let result = serde_json::to_value(&text_edits).unwrap();
    lsp_server::Response {
        id,
        result: Some(result),
        error: None,
    }
}
