use escpos::{driver::Driver, printer::Printer, utils::JustifyMode};

pub(super) fn print(printer: &mut Printer<impl Driver>, input: &str) -> escpos::errors::Result<()> {
    printer.reset_size()?.justify(JustifyMode::LEFT)?;

    for line in input.lines() {
        printer.writeln(line)?.print()?;
    }
    Ok(())
}
