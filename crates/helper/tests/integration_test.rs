use helper::{
    self,
    convert::{
        lsp_position_to_ts_point, lsp_range_to_ts_range, offset_to_position, position_to_offset,
        ts_point_to_lsp_position, ts_range_to_lsp_range,
    },
};
use lsp_types::Position;
use tree_sitter::Point;

#[test]
fn test_position_to_offset_and_revert_back() {
    let text = "12345678
    12345678
    12345678";
    let point = Point { row: 1, column: 3 };
    let offset = position_to_offset(text.as_bytes(), point);
    assert_eq!(offset, 12);

    assert_eq!(offset_to_position(text.as_bytes(), offset), point);
}

#[test]
fn ts_and_lsp_point() {
    // zero-based, so simple conversion
    let ts_point = Point { row: 1, column: 3 };
    let lsp_position = Position {
        line: 1,
        character: 3,
    };
    assert_eq!(ts_point_to_lsp_position(&ts_point), lsp_position);
    assert_eq!(lsp_position_to_ts_point(&lsp_position), ts_point);
}

#[test]
fn ts_range_and_lsp_range() {
    let text = "12345678
    12345678
    12345678";
    let ts_range = tree_sitter::Range {
        start_byte: 12,
        end_byte: 26,
        start_point: Point { row: 1, column: 3 },
        end_point: Point { row: 2, column: 4 },
    };
    let lsp_range = lsp_types::Range {
        start: Position {
            line: 1,
            character: 3,
        },
        end: Position {
            line: 2,
            character: 4,
        },
    };
    assert_eq!(ts_range_to_lsp_range(&ts_range), lsp_range);
    assert_eq!(lsp_range_to_ts_range(&lsp_range, text.as_bytes()), ts_range);
}
