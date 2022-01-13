pub mod errors;
pub mod highlight;
pub mod locals;
pub mod utils;

use tree_sitter::Node;

fn match_by_query_source<'tree>(
    source_code: &lsp_types::TextDocumentItem,
    node: Node<'tree>,
    query_source: &str,
) -> Vec<Vec<(String, Node<'tree>)>> {
    let query = tree_sitter::Query::new(node.language(), query_source).unwrap();

    // TODO: Store all the info and do query
    let mut query_cursor = tree_sitter::QueryCursor::new();
    let matches = query_cursor.matches(&query, node, source_code.text.as_bytes());

    matches
        .map(|m| {
            m.captures
                .iter()
                .map(|c| {
                    (
                        query.capture_names()[c.index as usize].as_str().to_owned(),
                        c.node.to_owned(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn capture_by_query_source<'tree>(
    source_code: String,
    node: Node<'tree>,
    query_source: &str,
) -> Vec<(String, Node<'tree>)> {
    let query = tree_sitter::Query::new(node.language(), query_source).unwrap();

    // TODO: Store all the info and do query
    let mut query_cursor = tree_sitter::QueryCursor::new();
    let mut captures = query_cursor.captures(&query, node, source_code.as_bytes());

    let mut result: Vec<(String, Node<'tree>)> = Vec::new();
    for (mat, capture_index) in &mut captures {
        let capture = mat.captures[capture_index];
        let capture_name = &query.capture_names()[capture.index as usize];
        let node: Node<'tree> = capture.node;
        result.push((capture_name.to_owned(), node));
    }

    result
}
