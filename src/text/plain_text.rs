use super::{Lines, TextType};

pub(super) struct PlainText {}

impl TextType for PlainText {
    fn detect(_: &Lines) -> bool {
        true
    }

    fn format(lines: Lines) -> Lines {
        lines
    }
}
