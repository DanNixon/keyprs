use escpos::{driver::Driver, printer::Printer, utils::JustifyMode};
use image::{ImageFormat, Luma};
use miette::IntoDiagnostic;
use qrcode::QrCode;
use std::io::Cursor;

pub(super) fn print(printer: &mut Printer<impl Driver>, input: &str) -> miette::Result<()> {
    let code = QrCode::new(input).into_diagnostic()?;

    let image = code.render::<Luma<u8>>().build();

    let mut bytes = Cursor::new(Vec::new());

    image
        .write_to(&mut bytes, ImageFormat::Png)
        .into_diagnostic()?;

    let bytes: Vec<u8> = bytes.into_inner();

    printer
        .reset_size()
        .into_diagnostic()?
        .justify(JustifyMode::CENTER)
        .into_diagnostic()?
        .bit_image_from_bytes(&bytes)
        .into_diagnostic()?;

    Ok(())
}
