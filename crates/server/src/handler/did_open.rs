use crate::global_state::GlobalState;
use helper::tree_mutator::get_parser;
use log::{debug, error};

pub fn did_open(params: lsp_types::DidOpenTextDocumentParams, global_state: &mut GlobalState) {
    debug!("Received a DidOpenTextDocument: {:?}", params);

    let mut parser = match get_parser(params.text_document.language_id.clone()) {
        Some(parser) => parser,
        None => {
            error!("Failed to get parser");
            return;
        }
    };
    let source_code = params.text_document.clone();

    let tree = match parser.parse(source_code.text, None) {
        Some(tree) => tree,
        None => {
            error!("Error while parsing");
            return;
        }
    };

    global_state.update_cache(params.text_document, &tree);
}
