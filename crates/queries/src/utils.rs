use lsp_types::{CompletionItemKind, Position};
use std::fs::File;
use std::io::prelude::*;
use tree_sitter::{Node, Point, Range};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub completion_kind: Vec<CompletionItemKind>,
    pub location: Range,
    pub belongs_to: Vec<Range>,
}

impl Symbol {
    pub fn default() -> Self {
        Self {
            name: String::new(),
            completion_kind: vec![CompletionItemKind::TEXT],
            location: Range {
                start_byte: 0,
                end_byte: 0,
                start_point: Point { row: 0, column: 0 },
                end_point: Point { row: 0, column: 0 },
            },
            belongs_to: vec![],
        }
    }
}
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

fn ts_point_to_lsp_position(point: &Point) -> Position {
    Position {
        line: point.row as u32,
        character: point.column as u32,
    }
}

/// Transform range from treesitter to lsp range. can't be reversed.
pub fn ts_range_to_lsp_range(range: &Range) -> lsp_types::Range {
    lsp_types::Range {
        start: lsp_types::Position {
            line: range.start_point.row as u32,
            character: range.start_point.column as u32,
        },
        end: lsp_types::Position {
            line: range.end_point.row as u32,
            character: range.end_point.column as u32,
        },
    }
}

pub fn get_query_source(language_id: &str, source_type: &str) -> Option<String> {
    // TODO: Hard-coded path for now

    let path = dirs::home_dir()
        .unwrap()
        .join(".local/share/nvim/site/pack/packer/opt/nvim-treesitter/queries/")
        .join(language_id)
        .join(source_type.to_owned() + ".scm");

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Some(contents)
}
