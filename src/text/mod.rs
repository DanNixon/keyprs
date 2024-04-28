mod age_secret_key;
mod plain_text;

use self::{age_secret_key::AgeSecretKey, plain_text::PlainText};

pub(super) type Lines = Vec<String>;

pub(super) fn process(lines: Lines) -> Lines {
    print!("🔑 The input looks like ");
    if AgeSecretKey::detect(&lines) {
        println!("an age secret key.\n");
        AgeSecretKey::format(lines)
    } else {
        println!("plain text.\n");
        PlainText::format(lines)
    }
}

trait TextType {
    fn detect(lines: &Lines) -> bool;
    fn format(lines: Lines) -> Lines;
}
