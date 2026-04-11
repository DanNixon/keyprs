use super::TextType;

pub(super) struct PgpArmor {}

impl TextType for PgpArmor {
    fn detect(input: &str) -> bool {
        input.contains("-----BEGIN PGP ") && input.contains("-----END PGP ")
    }

    fn format(input: &str, width: u8) -> String {
        let mut out_lines: Vec<String> = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            // Keep header and footer as-is.
            if line.contains("-----BEGIN PGP ") || line.contains("-----END PGP ") {
                out_lines.push(line.to_string());
                continue;
            }

            // Keep integrity/checksum lines (start with '=') as-is.
            if line.starts_with('=') {
                out_lines.push(line.to_string());
                continue;
            }

            // For payload lines, split into 4-character groups.
            out_lines.push(super::chunk_text_over_lines(line, 4, width as usize));
        }

        out_lines.join("\n\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect() {
        let fake_msg = "-----BEGIN PGP MESSAGE-----

jA0ECQMKM19m/RmQCCf30ukB6E/rJlb23wdDUHwee4TX8OoNp3JK4pOGxb7CXvFY
rMse20X4kArSb/mfaMjTOekdijcmIxhbF1inS8dhBsZ4vqvSeum8ctnGFnO7hmG1
PMSEYt1+rpwU
=B0fW
-----END PGP MESSAGE-----";
        assert!(PgpArmor::detect(fake_msg));
    }

    #[test]
    fn detect_plain_text() {
        let text = "not a pgp armored string";
        assert!(!PgpArmor::detect(text));
    }

    #[test]
    fn format() {
        let fake_msg = "-----BEGIN PGP MESSAGE-----

jA0ECQMKM19m/RmQCCf30ukB6E/rJlb23wdDUHwee4TX8OoNp3JK4pOGxb7CXvFY
rMse20X4kArSb/mfaMjTOekdijcmIxhbF1inS8dhBsZ4vqvSeum8ctnGFnO7hmG1
PMSEYt1+rpwU
=B0fW
-----END PGP MESSAGE-----";

        assert_eq!(
            PgpArmor::format(fake_msg, 42),
            "-----BEGIN PGP MESSAGE-----

jA0E CQMK M19m /RmQ CCf3 0ukB 6E/r Jlb2
3wdD UHwe e4TX 8OoN p3JK 4pOG xb7C XvFY

rMse 20X4 kArS b/mf aMjT Oekd ijcm Ixhb
F1in S8dh BsZ4 vqvS eum8 ctnG FnO7 hmG1

PMSE Yt1+ rpwU

=B0fW

-----END PGP MESSAGE-----"
        );
    }
}
