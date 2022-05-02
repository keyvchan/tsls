use helper::tree_mutator::get_parser;
use lsp_types::{error_codes::REQUEST_CANCELLED, TextEdit};
use tree_sitter::Tree;

use crate::{match_by_query_source, utils::get_query_source};

pub fn text_edits(text: Vec<u8>, language: &str, old_tree: &Tree) -> Result<Vec<TextEdit>, i64> {
    let parser = match get_parser(language.to_string()) {
        Some(parser) => parser,
        None => return Err(REQUEST_CANCELLED),
    };

    // we don't need to reparse the tree if the code hasn't changed
    let query_source = match get_query_source(language, "indents") {
        Some(query_source) => query_source,
        None => return Err(REQUEST_CANCELLED),
    };

    match_by_query_source(&text, old_tree.root_node(), &query_source);

    Ok(vec![])
}
