use helper::tree_mutator::get_parser;
use log::debug;
use lsp_types::TextEdit;
use tree_sitter::Tree;

use crate::{capture_by_query_source, utils::get_query_source};

pub fn text_edits(
    text: &Vec<u8>,
    language: &str,
    old_tree: &Tree,
) -> Result<Vec<TextEdit>, String> {
    let parser = match get_parser(language.to_string()) {
        Some(parser) => parser,
        None => return Err("No parser found for language".to_string()),
    };

    // we don't need to reparse the tree if the code hasn't changed
    let query_source = match get_query_source(language, "indents") {
        Some(query_source) => query_source,
        None => return Err("No query source found".to_string()),
    };

    for (capture, node) in capture_by_query_source(text, old_tree.root_node(), &query_source) {
        debug!("capture: {:?}, {:?}", capture, node);
    }

    Ok(vec![])
}
