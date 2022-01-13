/// This Module contains some helper functions that does type conversions.
///
/// # Functions List
///     - `tree_sitter::Point` -> Byte Offset
///     - Byte Offset -> `tree_sitter::Point`
///     - `tree_sitter::Point` -> `lsp_types::Position`
///     - `lsp_types::Position` -> `tree_sitter::Point`
///     - `tree_sitter::Range` -> `lsp_types::Range`
///     - `lsp_types::Range` -> `tree_sitter::Range`
///
pub mod convert {
    use lsp_types::Position;
    use tree_sitter::{Point, Range};

    pub fn position_to_offset(input: &[u8], position: Point) -> usize {
        let mut current_position = Point { row: 0, column: 0 };
        for (i, c) in input.iter().enumerate() {
            if *c as char == '\n' {
                current_position.row += 1;
                current_position.column = 0;
            } else {
                current_position.column += 1;
            }
            if current_position > position {
                return i;
            }
        }
        input.len()
    }

    pub fn offset_to_position(input: &[u8], offset: usize) -> Point {
        let mut result = Point { row: 0, column: 0 };
        for c in &input[0..offset] {
            if *c as char == '\n' {
                result.row += 1;
                result.column = 0;
            } else {
                result.column += 1;
            }
        }
        result
    }

    pub fn ts_point_to_lsp_position(point: &Point) -> Position {
        Position {
            line: point.row as u32,
            character: point.column as u32,
        }
    }

    pub fn lsp_position_to_ts_point(position: &Position) -> Point {
        Point {
            row: position.line as usize,
            column: position.character as usize,
        }
    }

    /// Transform range from treesitter to lsp range.
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

    pub fn lsp_range_to_ts_range(range: &lsp_types::Range, input: &[u8]) -> Range {
        Range {
            start_point: Point {
                row: range.start.line as usize,
                column: range.start.character as usize,
            },
            end_point: Point {
                row: range.end.line as usize,
                column: range.end.character as usize,
            },
            start_byte: position_to_offset(
                input,
                Point {
                    row: range.start.line as usize,
                    column: range.start.character as usize,
                },
            ),
            end_byte: position_to_offset(
                input,
                Point {
                    row: range.end.line as usize,
                    column: range.end.character as usize,
                },
            ),
        }
    }
}

/// Module tree_mutator contains functions that mutate the tree.
pub mod tree_mutator {

    use log::{debug, error};
    use tree_sitter::{InputEdit, Language, Parser, Tree};

    /// Perform an edit on the tree.
    pub fn perform_edit(tree: &mut Tree, edit: &InputEdit) {
        tree.edit(edit);
    }

    /// Get parser for the given language.
    pub fn get_parser(language_id: String) -> Option<Parser> {
        let mut parser = Parser::new();

        // TODO: Default language to plain text
        // Matching the language for all kind of parser, we read the config then determ which language
        // should be enabled
        let language: Language = match language_id.as_str() {
            "c" => tree_sitter_c::language(),
            "cpp" => tree_sitter_cpp::language(),
            _ => {
                error!("Language not supported");
                // Set fallback to plain text
                return None;
            }
        };
        match parser.set_language(language) {
            Ok(_) => {
                debug!("Language set");
            }
            Err(e) => {
                error!("Error while setting language: {}", e);
                return None;
            }
        };

        Some(parser)
    }
}

/// Module types contains useful types for representing the source code.
pub mod types {
    use lsp_types::CompletionItemKind;
    use tree_sitter::{Point, Range};

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
}
