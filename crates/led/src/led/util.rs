/// Counts the number of line breaks (`'\n'` characters) in the given text.
///
/// # Arguments
///
/// * `text` - A reference to a `String` containing the text to search.
///
/// # Returns
///
/// The number of line breaks as a `u32`.
pub(crate) fn count_line_breaks(text: &String) -> u32 {
    text.chars().filter(|&c| c == '\n').count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_no_line_breaks_in_empty_string() {
        let text = String::from("");
        assert_eq!(count_line_breaks(&text), 0);
    }

    #[test]
    fn counts_no_line_breaks_in_single_line() {
        let text = String::from("Hello, world!");
        assert_eq!(count_line_breaks(&text), 0);
    }

    #[test]
    fn counts_single_line_break() {
        let text = String::from("Hello\nworld!");
        assert_eq!(count_line_breaks(&text), 1);
    }

    #[test]
    fn counts_multiple_line_breaks() {
        let text = String::from("a\nb\nc\nd");
        assert_eq!(count_line_breaks(&text), 3);
    }

    #[test]
    fn counts_line_breaks_at_start_and_end() {
        let text = String::from("\nHello\nworld!\n");
        assert_eq!(count_line_breaks(&text), 3);
    }

    #[test]
    fn counts_only_newline_characters() {
        let text = String::from("line1\r\nline2\r\nline3");
        // Only '\n' is counted, not '\r'
        assert_eq!(count_line_breaks(&text), 2);
    }
}