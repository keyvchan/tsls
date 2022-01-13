pub mod convert {
    use tree_sitter::Point;

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
}

pub mod operation {

    use log::{debug, error};
    use tree_sitter::{InputEdit, Language, Parser, Tree};

    pub fn perform_edit(tree: &mut Tree, edit: &InputEdit) {
        tree.edit(edit);
    }

    pub fn get_parser(language_id: String) -> Option<Parser> {
        let mut parser = Parser::new();

        // Default language is C
        let mut language: Language = tree_sitter_c::language();
        // Matching the language for all kind of parser, we read the config then determ which language
        // should be enabled
        match language_id.as_str() {
            "c" => {
                language = tree_sitter_c::language();
            }
            "cpp" => {
                language = tree_sitter_cpp::language();
            }
            _ => {
                error!("Language not supported");
                // Set fallback to plain text
                return None;
            }
        }
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
