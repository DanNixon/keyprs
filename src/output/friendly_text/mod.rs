mod age_secret_key;
mod pgp_armor;

use self::{age_secret_key::AgeSecretKey, pgp_armor::PgpArmor};
use escpos::{driver::Driver, printer::Printer, utils::JustifyMode};
use miette::IntoDiagnostic;

trait TextType {
    fn detect(lines: &str) -> bool;
    fn format(lines: &str, width: u8) -> String;
}

pub(super) fn print(printer: &mut Printer<impl Driver>, input: &str) -> miette::Result<()> {
    let width = printer.options().get_characters_per_line();

    let output = if AgeSecretKey::detect(input) {
        AgeSecretKey::format(input, width)
    } else if PgpArmor::detect(input) {
        PgpArmor::format(input, width)
    } else {
        return Err(miette::miette!(
            "No friendly representation for provided secret"
        ));
    };

    printer.justify(JustifyMode::LEFT).into_diagnostic()?;
    for line in output.lines() {
        printer
            .writeln(line)
            .into_diagnostic()?
            .print()
            .into_diagnostic()?;
    }

    Ok(())
}

fn chunk_text_over_lines(input: &str, chunk_size: usize, line_width: usize) -> String {
    // Split the input into characters and then into chunks of the specified size.
    let chars: Vec<char> = input.chars().collect();
    let chunks = chars.chunks(chunk_size).collect::<Vec<_>>();

    // Determine how many chunks can fit on a line based on the line width and chunk size (plus one space).
    let per_line = line_width / (chunk_size + 1);

    // Group the chunks into lines and join them with spaces, then join the lines with newlines.
    chunks
        .chunks(per_line)
        .map(|line_chunks| {
            line_chunks
                .iter()
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
