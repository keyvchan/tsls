use helper::{
    convert::{offset_to_position, position_to_offset},
    tree_mutator::{get_parser, perform_edit},
};
use log::{debug, error};
use lsp_types::{self, DidChangeTextDocumentParams};
use tree_sitter::{InputEdit, Point};

use crate::global_state::GlobalState;

pub fn did_change(params: DidChangeTextDocumentParams, global_state: &mut GlobalState) {
    let language_id = global_state
        .get_language_id(&params.text_document.uri)
        .unwrap_or_default();
    let mut parser = match get_parser(language_id.clone()) {
        Some(parser) => parser,
        None => {
            error!("No parser found for language");
            return;
        }
    };

    // Check version
    if params.text_document.version
        <= global_state
            .get_version(&params.text_document.uri)
            .unwrap_or(0)
    {
        error!("Received outdated version of text document");
        return;
    }

    if params.content_changes.is_empty() {
        // No changes
        return;
    }

    // update cache
    let mut source_code = global_state
        .get_source_code(&params.text_document.uri)
        .unwrap_or_else(|| "".as_bytes().to_vec());
    let mut start_byte;
    let mut end_byte;
    let mut start_position;
    let mut end_position;

    let mut edit = InputEdit {
        start_byte: 0,
        old_end_byte: 0,
        new_end_byte: 0,
        start_position: Point { row: 0, column: 0 },
        old_end_position: Point { row: 0, column: 0 },
        new_end_position: Point { row: 0, column: 0 },
    };

    // copy to a new tree
    let mut old_tree = match global_state.get_tree(&params.text_document.uri) {
        Some(tree) => tree.clone(),
        None => return,
    };

    // Update the edit object
    for change in params.content_changes {
        let range = change.range.unwrap_or_default();
        let content = change.text;

        // calculate the start and end byte
        start_byte = position_to_offset(
            &source_code,
            Point::new(range.start.line as usize, range.start.character as usize),
        );
        end_byte = position_to_offset(
            &source_code,
            Point::new(range.end.line as usize, range.end.character as usize),
        );

        // calculate the start_point and end_point
        start_position = Point::new(range.start.line as usize, range.start.character as usize);
        end_position = Point::new(range.end.line as usize, range.end.character as usize);

        error!("start_byte: {}, end_byte: {}", start_byte, end_byte);

        // start_position just needs to set to the start
        // if start_byte < 0, start_byte = 0
        edit.start_byte = if start_byte < edit.start_byte {
            usize::MIN
        } else {
            start_byte
        };

        edit.start_position = if (start_position) < edit.start_position {
            edit.start_position
        } else {
            {
                let row = range.start.line as usize;
                let column = range.start.character as usize;
                Point { row, column }
            }
        };

        // Deletion/Modification
        // Content is is_empty stands for deletion
        // if content is not empty, it is modification
        if content.is_empty() {
            // Deletion
            edit.old_end_byte = end_byte;
            edit.new_end_byte = start_byte;

            edit.old_end_position = end_position;
            edit.new_end_position = start_position;

            // edit the source_code
            source_code.drain(start_byte..end_byte);
        } else {
            // Modification
            edit.old_end_byte = start_byte;
            edit.new_end_byte = end_byte + content.len();

            // edit the source_code
            source_code.splice(start_byte..end_byte, content.as_bytes().to_vec());

            // edit the old_end_position
            edit.new_end_position = offset_to_position(&source_code, end_byte + content.len() - 1);
            edit.old_end_position = start_position;
        }

        // fixed: index out of range on symbol modified
        debug!("InputEdit: {:?}", edit);
        // edit tree each rounds
        perform_edit(&mut old_tree, &edit);
    }

    // update cache
    global_state.update_source_code(&params.text_document.uri, source_code.clone());

    // Use final source code and final tree to generate new AST
    let new_tree = match parser.parse(source_code.clone(), Some(&old_tree)) {
        Some(tree) => tree,
        None => return,
    };

    global_state.update_cache(
        lsp_types::TextDocumentItem {
            language_id,
            uri: params.text_document.uri.to_owned(),
            version: params.text_document.version,
            text: String::from_utf8(source_code).unwrap(),
        },
        &new_tree,
    );
}
