use crate::global_state::GlobalState;
use helper::{
    convert::position_to_offset,
    tree_mutator::{get_parser, perform_edit},
};
use log::{debug, error};
use lsp_types::{self, DidChangeTextDocumentParams};
use tree_sitter::{InputEdit, Point};

pub fn did_change(params: DidChangeTextDocumentParams, global_state: &mut GlobalState) {
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

    let mut edit = InputEdit {
        start_byte: 0,
        old_end_byte: 0,
        new_end_byte: 0,
        start_position: Point { row: 0, column: 0 },
        old_end_position: Point { row: 0, column: 0 },
        new_end_position: Point { row: 0, column: 0 },
    };

    // Update the edit object
    for change in params.content_changes {
        let range = change.range.unwrap_or_default();
        let content = change.text;

        start_byte = position_to_offset(
            &source_code,
            Point::new(range.start.line as usize, range.start.character as usize),
        );
        end_byte = position_to_offset(
            &source_code,
            Point::new(range.end.line as usize, range.end.character as usize),
        );

        // check start_byte < 0
        edit.start_byte = if start_byte < edit.start_byte {
            start_byte
        } else {
            edit.start_byte
        };
        edit.new_end_byte = if end_byte > edit.new_end_byte {
            end_byte
        } else {
            edit.new_end_byte
        };

        edit.start_position =
            if Point::new(range.start.line as usize, range.start.character as usize)
                < edit.start_position
            {
                Point::new(range.start.line as usize, range.start.character as usize)
            } else {
                edit.start_position
            };
        edit.new_end_position = if Point::new(range.end.line as usize, range.end.character as usize)
            > edit.new_end_position
        {
            Point::new(range.end.line as usize, range.end.character as usize)
        } else {
            edit.new_end_position
        };

        // old_end_byte set to start_byte stands for no deletion
        edit.old_end_byte = start_byte;
        edit.old_end_position =
            Point::new(range.start.line as usize, range.start.character as usize);

        // It's a deletion
        if content.is_empty() {
            debug!("Deletion");
            // remove this element, since the end byte is exclusive, we use end_byte - 1 in here.
            edit.old_end_byte = if end_byte > edit.old_end_byte {
                // FIX: fixed a crash bug when deleting a character
                end_byte - 1
            } else {
                edit.old_end_byte
            };
            edit.old_end_position =
                if Point::new(range.end.line as usize, range.end.character as usize)
                    > edit.old_end_position
                {
                    Point::new(range.end.line as usize, range.end.character as usize)
                } else {
                    edit.old_end_position
                };

            // delete the content
            source_code.drain(start_byte..end_byte);
        } else {
            debug!("Modification");

            edit.new_end_byte = if end_byte > edit.new_end_byte {
                end_byte
            } else {
                edit.new_end_byte - 1
            };

            source_code.splice(start_byte..end_byte, content.as_bytes().to_vec());
        }
    }

    // Now, we get the final source code
    debug!(
        "final source code: {:?}, {:?}",
        String::from_utf8(source_code.clone()).unwrap(),
        source_code.len()
    );

    // update cache
    global_state.update_source_code(&params.text_document.uri, source_code.clone());
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

    let old_tree = match global_state.get_mutable_tree(&params.text_document.uri) {
        Some(tree) => tree,
        None => return,
    };

    // fixed: index out of range on symbol modified
    perform_edit(old_tree, &edit);
    //
    let new_tree = match parser.parse(source_code.clone(), Some(old_tree)) {
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
