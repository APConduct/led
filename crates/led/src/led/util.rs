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