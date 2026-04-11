use super::TextType;

const PREFIX: &str = "AGE-SECRET-KEY-";

pub(super) struct AgeSecretKey {}

impl TextType for AgeSecretKey {
    fn detect(input: &str) -> bool {
        if input.lines().count() != 1 {
            false
        } else {
            input.starts_with(PREFIX)
        }
    }

    fn format(input: &str, width: u8) -> String {
        // Extract the part after the prefix
        let s = input.strip_prefix(PREFIX).unwrap_or("").to_string();

        // Chunk the key
        let formatted_key = super::chunk_text_over_lines(&s, 4, width as usize);

        format!("{PREFIX}\n{formatted_key}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect() {
        let fake_key = "AGE-SECRET-KEY-1WYJ8E9DMQQED73R2L4W0MVLK6AFANQUJ0PVV8YFVXYWUULP99LGSZRX9CK";
        assert!(AgeSecretKey::detect(fake_key));
    }

    #[test]
    fn detect_multiline() {
        let fake_key = "AGE-SECRET-KEY-1WYJ8E9DMQQED73R2L4W0MVLK6AFANQUJ0PVV8YFVXYWUULP99LGSZRX9CK
AGE-SECRET-KEY-1WYJ8E9DMQQED73R2L4W0MVLK6AFANQUJ0PVV8YFVXYWUULP99LGSZRX9CK";
        assert!(!AgeSecretKey::detect(fake_key));
    }

    #[test]
    fn detect_plain_text() {
        let text = "not an age secret key";
        assert!(!AgeSecretKey::detect(text));
    }

    #[test]
    fn format() {
        let fake_key = "AGE-SECRET-KEY-1WYJ8E9DMQQED73R2L4W0MVLK6AFANQUJ0PVV8YFVXYWUULP99LGSZRX9CK";
        let output = AgeSecretKey::format(fake_key, 42);
        assert_eq!(
            output,
            "AGE-SECRET-KEY-
1WYJ 8E9D MQQE D73R 2L4W 0MVL K6AF ANQU
J0PV V8YF VXYW UULP 99LG SZRX 9CK",
        );
    }
}
