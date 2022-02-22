pub use embed::get_query_source;
use helper::convert::ts_point_to_lsp_position;
use lsp_types::Position;
use tree_sitter::{Node, Range};

pub fn get_smallest_scope_id_by_position(p: &Position, scopes: &[Range]) -> usize {
    let mut scope_id: usize = 0;
    for (pos, this_scope) in scopes.iter().enumerate() {
        let start_position = ts_point_to_lsp_position(&this_scope.start_point);
        let end_position = ts_point_to_lsp_position(&this_scope.end_point);

        if p > &start_position && p < &end_position {
            scope_id = pos;
        }
    }
    scope_id
}

pub fn get_smallest_scope_id_by_node(node: &Node, scopes: &[Range]) -> usize {
    let node_range = node.range();
    let mut positon: usize = 0;
    for (pos, scope) in scopes.iter().enumerate() {
        // we can't use range cmp here.
        // we check start_byte and end_byte
        if node_range.start_byte > scope.start_byte && node_range.end_byte < scope.end_byte {
            if pos + 1 == scopes.len() {
                return pos;
            } else {
                positon = pos;
            }
        } else {
            continue;
        }
    }
    positon
}

pub mod embed {
    use log::debug;
    // embed
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "../../queries/nvim-treesitter/queries"]
    #[prefix = "basic/"]
    struct Asset;

    /// Get queries in embeded files
    pub fn get_query_source(language_id: &str, source_type: &str) -> Option<String> {
        let path = std::path::Path::new("basic/")
            .join(language_id)
            .join(source_type.to_owned() + ".scm");
        debug!(
            "get_query_source: language_id: {}, source_type: {}, path: {:?}",
            language_id, source_type, path
        );

        let file = Asset::get(path.to_str().unwrap()).unwrap();
        let contents = String::from_utf8(file.data.as_ref().to_vec()).unwrap();
        Some(contents)
    }
}
