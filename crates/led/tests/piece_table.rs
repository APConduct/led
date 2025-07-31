use led::piece_table::piece::Table;
use led::types::Position;

fn make_table_with_text(text: &str) -> Table {
    let mut table = Table::new("".to_string());
    let _ = table.insert(0, text);
    table
}

#[test]
fn test_insert_and_get_text() {
    let mut table = Table::new("".to_string());
    assert_eq!(table.len(), 0);
    let _ = table.insert(0, "Hello");
    assert_eq!(table.get_text(0, table.len()), "Hello");
    let _ = table.insert(5, ", world!");
    assert_eq!(table.get_text(0, table.len()), "Hello, world!");
}

#[test]
fn test_delete() {
    let mut table = make_table_with_text("Hello, world!");
    // Delete ", "
    let _ = table.delete(5, 2);
    assert_eq!(table.get_text(0, table.len()), "Helloworld!");
    // Delete "world"
    let _ = table.delete(5, 5);
    assert_eq!(table.get_text(0, table.len()), "Hello!");
}

#[test]
fn test_offset_to_position_and_back() {
    let text = "Hello\nworld\npiece\ntable";
    let table = make_table_with_text(text);

    // Check offset_to_position
    let pos = table.offset_to_position(0);
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);

    let pos = table.offset_to_position(6); // after 'Hello\n'
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 0);

    let pos = table.offset_to_position(12); // after 'Hello\nworld\n'
    assert_eq!(pos.line, 2);
    assert_eq!(pos.column, 0);

    // Check position_to_offset
    let offset = table.position_to_offset(Position { line: 0, column: 0 });
    assert_eq!(offset, 0);

    let offset = table.position_to_offset(Position { line: 1, column: 0 });
    assert_eq!(offset, 6);

    let offset = table.position_to_offset(Position { line: 2, column: 0 });
    assert_eq!(offset, 12);

    let offset = table.position_to_offset(Position { line: 3, column: 2 });
    assert_eq!(offset, 19); // 'Hello\nworld\npiece\nta'
}

#[test]
fn test_lines_count() {
    let text = "a\nb\nc";
    let table = make_table_with_text(text);
    assert_eq!(table.lines(), 3);

    let mut table = table;
    let _ = table.insert(table.len(), "\nd");
    assert_eq!(table.lines(), 4);

    let _ = table.delete(0, 2); // delete 'a\n'
    assert_eq!(table.lines(), 3);
}

#[test]
fn test_insert_delete_multiline() {
    let mut table = Table::new("".to_string());
    let _ = table.insert(0, "foo\nbar\nbaz");
    assert_eq!(table.lines(), 3);

    let _ = table.insert(4, "X\nY\n");
    assert_eq!(table.get_text(0, table.len()), "foo\nX\nY\nbar\nbaz");
    assert_eq!(table.lines(), 5);

    let _ = table.delete(4, 4); // delete "X\nY\n"
    assert_eq!(table.get_text(0, table.len()), "foo\nbar\nbaz");
    assert_eq!(table.lines(), 3);
}
