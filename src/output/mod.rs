mod friendly_text;
mod qr_code;
mod raw_text;

use clap::ValueEnum;
use escpos::{driver::Driver, printer::Printer, utils::JustifyMode};
use miette::IntoDiagnostic;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(super) enum OutputMode {
    RawText,
    FriendlyText,
    QrCode,
}

pub(super) trait PrinterExt {
    fn print_secret(&mut self, mode: OutputMode, input: &str) -> miette::Result<()>;
}

impl<D: Driver> PrinterExt for Printer<D> {
    fn print_secret(&mut self, mode: OutputMode, input: &str) -> miette::Result<()> {
        self.reset_size()
            .into_diagnostic()?
            .justify(JustifyMode::CENTER)
            .into_diagnostic()?
            .writeln(match mode {
                OutputMode::RawText => "=== Raw Text ===",
                OutputMode::FriendlyText => "=== Friendly Text ===",
                OutputMode::QrCode => "=== QR Code ===",
            })
            .into_diagnostic()?
            .print()
            .into_diagnostic()?;

        let input = input.trim();

        match mode {
            OutputMode::RawText => raw_text::print(self, input).into_diagnostic(),
            OutputMode::FriendlyText => friendly_text::print(self, input),
            OutputMode::QrCode => qr_code::print(self, input),
        }
    }
}
