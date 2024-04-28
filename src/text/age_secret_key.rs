use super::{Lines, TextType};

const PREFIX: &str = "AGE-SECRET-KEY-";

pub(super) struct AgeSecretKey {}

impl TextType for AgeSecretKey {
    fn detect(lines: &Lines) -> bool {
        if lines.len() != 1 {
            return false;
        }

        if let Some(first) = lines.first() {
            if first.starts_with(PREFIX) {
                return true;
            }
        }

        false
    }

    fn format(mut lines: Lines) -> Lines {
        let mut s = lines[0].split_off(PREFIX.len());

        let mut result = vec![PREFIX.to_string()];

        while !s.is_empty() {
            let remaining = s.split_off(std::cmp::min(s.len(), 4));
            result.push(s);
            s = remaining;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_key() {
        let fake_key = "AGE-SECRET-KEY-1WYJ8E9DMQQED73R2L4W0MVLK6AFANQUJ0PVV8YFVXYWUULP99LGSZRX9CK";
        let lines = vec![fake_key.to_string()];
        assert!(AgeSecretKey::detect(&lines));
    }

    #[test]
    fn detect_not_key() {
        let text = "not an age secret key";
        let lines = vec![text.to_string()];
        assert!(!AgeSecretKey::detect(&lines));
    }

    #[test]
    fn format() {
        let intentionally_shorter_fake_key = "AGE-SECRET-KEY-1WYJ8E9DMQQ";
        let lines = vec![intentionally_shorter_fake_key.to_string()];
        let lines = AgeSecretKey::format(lines);
        assert_eq!(
            lines,
            vec![
                "AGE-SECRET-KEY-".to_string(),
                "1WYJ".to_string(),
                "8E9D".to_string(),
                "MQQ".to_string(),
            ]
        );
    }
}
