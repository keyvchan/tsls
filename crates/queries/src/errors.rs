use helper::convert::ts_range_to_lsp_range;
use tree_sitter::Node;

use crate::match_by_query_source;

pub const ERROR: &str = "ERROR";

pub fn build_diagnostics(source_code: Vec<u8>, node: &Node) -> Vec<lsp_types::Diagnostic> {
    let result = match_by_query_source(&source_code, *node, "(ERROR) @ERROR");

    let mut errors: Vec<lsp_types::Diagnostic> = Vec::new();

    for item in &result {
        for (variable_type, node) in item {
            if variable_type == ERROR {
                let range = ts_range_to_lsp_range(&node.range());

                let diagnostic = lsp_types::Diagnostic::new_simple(range, "ERROR".to_string());

                errors.push(diagnostic);
            }
        }
    }

    errors
}
