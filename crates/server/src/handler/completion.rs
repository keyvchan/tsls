use helper::types::Symbol;
use log::debug;
use lsp_server::{ErrorCode::InternalError, RequestId, Response};
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionList, CompletionParams, CompletionResponse,
};
use queries::utils::get_smallest_scope_id_by_position;

use crate::global_state::GlobalState;

pub fn completion(id: RequestId, params: CompletionParams, state: GlobalState) -> Response {
    debug!("got completion request #{}: {:?}", id, params);

    // 1. we check the context
    let context = params.context.unwrap_or(lsp_types::CompletionContext {
        trigger_kind: lsp_types::CompletionTriggerKind::INVOKED,
        trigger_character: Some(".".to_string()),
    });

    let mut completion_items: Vec<CompletionItem> = Vec::new();
    // push all keywords to completion_items
    for keyword in state
        .sources
        .get(&params.text_document_position.text_document.uri)
        .unwrap()
        .keywords
        .iter()
    {
        completion_items.push(CompletionItem {
            label: keyword.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        });
    }

    match context.trigger_kind {
        lsp_types::CompletionTriggerKind::INVOKED => {
            // Return all identifiers.
            // we get the node

            let scope_id = get_smallest_scope_id_by_position(
                &params.text_document_position.position,
                &state
                    .sources
                    .get(&params.text_document_position.text_document.uri)
                    .unwrap()
                    .ordered_scopes,
            );
            debug!("scope id: {}", scope_id);

            let symbols = vec![Symbol::default()];
            let symbols = state
                .sources
                .get(&params.text_document_position.text_document.uri)
                .unwrap()
                .identifiers
                .get(&scope_id)
                .unwrap_or(&symbols);

            for symbol in symbols {
                match *symbol.completion_kind.last().unwrap() {
                    CompletionItemKind::OPERATOR => {}
                    _ => {
                        let completion_item = CompletionItem {
                            label: symbol.name.clone(),
                            kind: Some(*symbol.completion_kind.last().unwrap()),
                            ..Default::default()
                        };
                        completion_items.push(completion_item);
                    }
                }
            }
        }
        lsp_types::CompletionTriggerKind::TRIGGER_CHARACTER => {
            let trigger_character = context.trigger_character.unwrap_or_else(|| "".to_string());
            debug!("trigger character: {}", trigger_character);

            return Response::new_err(
                id,
                InternalError as i32,
                "trigger_character not implemented".to_string(),
            );
        }
        lsp_types::CompletionTriggerKind::TRIGGER_FOR_INCOMPLETE_COMPLETIONS => {
            return Response::new_err(
                id,
                InternalError as i32,
                "trigger_character not implemented".to_string(),
            );
        }

        _ => {}
    }

    // TODO: implement

    let result = Some(CompletionResponse::List(CompletionList {
        is_incomplete: false,
        items: completion_items,
    }));
    let result = serde_json::to_value(&result).unwrap();
    let resp = lsp_server::Response {
        id,
        result: Some(result),
        error: None,
    };
    debug!("send completion response {:?}", resp);
    resp
}
