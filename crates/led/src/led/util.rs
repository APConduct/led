pub(crate) fn count_line_breaks(text: &String) -> u32 {
    text.chars().filter(|&c| c == '\n').count() as u32
}
