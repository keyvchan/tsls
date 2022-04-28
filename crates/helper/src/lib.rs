/// This Module contains some helper functions that does type conversions.
///
/// # Functions List
/// `tree_sitter::Point` -> Byte Offset
/// Byte Offset -> `tree_sitter::Point`
/// `tree_sitter::Point` -> `lsp_types::Position`
/// `lsp_types::Position` -> `tree_sitter::Point`
/// `tree_sitter::Range` -> `lsp_types::Range`
/// `lsp_types::Range` -> `tree_sitter::Range`
///
pub mod convert {
    use lsp_types::Position;
    use tree_sitter::{Point, Range};

    /// Converts a `tree_sitter::Point` to byte offset
    ///
    /// # Examples
    /// ```rust
    /// use helper::convert::position_to_offset;
    /// use tree_sitter::Point;
    ///
    /// let text = r#"
    /// int main() {
    ///     printf("Hello World!");
    /// }
    /// "#;
    /// let point = Point { row: 1, column: 3 };
    /// let offset = position_to_offset(text.as_bytes(), point);
    /// ```
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

    /// Converts a byte offset to `tree_sitter::Point`
    /// # Examples
    /// ```rust
    /// use helper::convert::offset_to_position;
    /// use tree_sitter::Point;
    ///
    /// let text = r#"
    /// int main() {
    ///    printf("Hello World!");
    /// }
    /// "#;
    /// let offset = 12;
    /// let point = offset_to_position(text.as_bytes(), offset);
    /// ```
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

    /// Converts a `tree_sitter::Point` to `lsp_types::Position`
    ///
    /// # Examples
    /// ```rust
    /// use helper::convert::ts_point_to_lsp_position;
    /// use tree_sitter::Point;
    ///
    /// let point = Point { row: 1, column: 3 };
    /// let position = ts_point_to_lsp_position(&point);
    /// ```
    pub fn ts_point_to_lsp_position(point: &Point) -> Position {
        Position {
            line: point.row as u32,
            character: point.column as u32,
        }
    }

    /// Converts a `lsp_types::Position` to `tree_sitter::Point`
    ///
    /// # Examples
    /// ```rust
    /// use helper::convert::lsp_position_to_ts_point;
    /// use lsp_types::Position;
    ///
    /// let position = Position { line: 1, character: 3 };
    /// let point = lsp_position_to_ts_point(&position);
    /// ```
    pub fn lsp_position_to_ts_point(position: &Position) -> Point {
        Point {
            row: position.line as usize,
            column: position.character as usize,
        }
    }

    /// Converts `tree_sitter::Range` to `lsp_types::Range`.
    /// # Examples
    /// ```rust
    /// use helper::convert::ts_range_to_lsp_range;
    /// use tree_sitter::{Range, Point};
    ///
    /// let ts_range = tree_sitter::Range {
    ///     start_byte: 12,
    ///     end_byte: 26,
    ///     start_point: Point { row: 1, column: 3 },
    ///     end_point: Point { row: 2, column: 4 },
    /// };
    /// let lsp_range = ts_range_to_lsp_range(&ts_range);
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

    /// Converts `lsp_types::Range` to `tree_sitter::Range`.
    /// # Examples
    /// ```rust
    /// use helper::convert::lsp_range_to_ts_range;
    /// use lsp_types::{Range, Position};
    ///
    /// let text = r#"
    /// int main() {
    ///    printf("Hello World!");
    ///    return 0;
    /// }
    /// "#;
    /// let lsp_range = Range {
    ///    start: Position { line: 1, character: 3 },
    ///    end: Position { line: 2, character: 4 },
    ///};
    /// let ts_range = lsp_range_to_ts_range(&lsp_range, text.as_bytes());
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
            // On crates.io
            "c" => tree_sitter_c::language(),
            "cpp" => tree_sitter_cpp::language(),
            "rust" => tree_sitter_rust::language(),
            "python" => tree_sitter_python::language(),
            "javascript" => tree_sitter_javascript::language(),
            "typescript" => tree_sitter_typescript::language_typescript(),
            "go" => tree_sitter_go::language(),
            "cuda" => tree_sitter_cuda::language(),
            "kotlin" => tree_sitter_kotlin::language(),
            "glsl" => tree_sitter_glsl::language(),
            "hare" => tree_sitter_hare::language(),
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
    use lsp_types::{CompletionItemKind, SymbolKind};
    use tree_sitter::{Point, Range};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Symbol {
        pub name: String,
        pub completion_kind: Vec<CompletionItemKind>,
        pub symbol_kind: Vec<SymbolKind>,
        pub location: Range,

        // children could be None or multiple symbols
        pub children: Option<Vec<Symbol>>,
        pub belongs_to_scopes: Vec<Range>,
    }

    impl Symbol {
        pub fn default() -> Self {
            Self {
                name: String::new(),
                completion_kind: vec![CompletionItemKind::TEXT],
                symbol_kind: vec![SymbolKind::STRING],
                location: Range {
                    start_byte: 0,
                    end_byte: 0,
                    start_point: Point { row: 0, column: 0 },
                    end_point: Point { row: 0, column: 0 },
                },
                children: None,
                belongs_to_scopes: vec![],
            }
        }

        /// Get a reference to the symbol's belongs to.
        pub fn belongs_to(&self) -> &[Range] {
            self.belongs_to_scopes.as_ref()
        }
    }
}

pub mod tree_walker {

    use lsp_types::Position;
    use tree_sitter::{Node, Tree};

    use crate::convert::lsp_position_to_ts_point;

    pub fn get_named_node_by_position(tree: &Tree, position: Position) -> Option<Node> {
        let point = lsp_position_to_ts_point(&position);

        let root_node = tree.root_node();
        let tree_cursor = root_node.walk();
        tree_cursor
            .node()
            .named_descendant_for_point_range(point, point)
    }
}
