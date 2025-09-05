mod printer;
mod text;

use crate::text::Lines;
use clap::{Args, Parser};
use escpos::{
    driver::{Driver, SerialPortDriver},
    printer::Printer,
    utils::{JustifyMode, Protocol},
};
use miette::{miette, IntoDiagnostic, Result};
use std::{io::BufRead, time::Duration};

/// Format and print secrets using dumb receipt/POS printers.
///
/// Secret material should be passed via stdin, it will be inspected and an attempt at
/// understanding its type made, based on the type it will then be reformatted in a manner that
/// allows easy manual reentry if/when required.
///
/// The following secret types are understood, everything else will be assumed to be plain text:
///   - age secret keys
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// An optional node to add to the printout, can be used as an identifier
    #[arg(short, long)]
    note: Option<String>,

    #[clap(flatten)]
    printer: SerialPrinterOptions,
}

#[derive(Debug, Args)]
struct SerialPrinterOptions {
    /// The serial port that the printer is connected to
    #[arg(long)]
    serial_port: Option<String>,

    /// The baud rate the printer is expecting serial communications at
    #[arg(long, default_value = "9600")]
    serial_baud: u32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input_lines = std::io::stdin()
        .lock()
        .lines()
        .map(|i| i.unwrap())
        .collect();

    let lines = generate_output(cli.note, input_lines);

    let mut printer = if let Some(port) = cli.printer.serial_port {
        let driver =
            SerialPortDriver::open(&port, cli.printer.serial_baud, Some(Duration::from_secs(5)))
                .into_diagnostic()?;
        let mut printer = Printer::new(driver.clone(), Protocol::default(), None);
        init_and_check_printer(&driver, &mut printer)?;
        Some(printer)
    } else {
        None
    };

    if let Some(printer) = printer.as_mut() {
        printer.justify(JustifyMode::LEFT).into_diagnostic()?;
        printer.reset_size().into_diagnostic()?;
    }

    println!(
        "📃 I {} print the following:",
        match printer {
            Some(_) => "will",
            None => "would",
        }
    );

    println!("~~~");
    for line in lines {
        println!("{line}");
        if let Some(printer) = printer.as_mut() {
            printer.writeln(&line).into_diagnostic()?;
        }
    }
    println!("~~~");

    if let Some(printer) = printer.as_mut() {
        printer.reset_size().into_diagnostic()?;
        printer.feed().into_diagnostic()?;
        printer.print_cut().into_diagnostic()?;

        println!();
        println!("👀 Be sure to verify the printed output matches the above text!");
    }

    Ok(())
}

fn init_and_check_printer<D: Driver>(driver: &D, printer: &mut Printer<D>) -> Result<()> {
    printer.init().into_diagnostic()?;

    println!("🔍 Checking printer is ready...");
    if printer::check_online(driver, printer).into_diagnostic()? {
        println!("✅ Printer is online.");
        println!();
        Ok(())
    } else {
        Err(miette!("Printer is offline"))
    }
}

fn generate_output(note: Option<String>, input: Lines) -> Lines {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z");
    let mut lines = vec![format!("Printed: {timestamp}")];

    if let Some(note) = note {
        lines.push(format!("Note: {note}"));
    }

    lines.push(String::default());

    lines.append(&mut self::text::process(input));

    lines
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn output_plain_note() {
        let note = Some("some note".to_string());
        let lines = vec!["some secret plain text".to_string()];

        let lines = generate_output(note, lines);

        assert_eq!(lines.len(), 4);
        assert!(lines[0].starts_with("Printed: "));
        assert_eq!(lines[1], "Note: some note");
        assert_eq!(lines[2], "");
        assert_eq!(lines[3], "some secret plain text");
    }

    #[test]
    fn output_plain_no_note() {
        let note = None;
        let lines = vec!["some secret plain text".to_string()];

        let lines = generate_output(note, lines);

        assert_eq!(lines.len(), 3);
        assert!(lines[0].starts_with("Printed: "));
        assert_eq!(lines[1], "");
        assert_eq!(lines[2], "some secret plain text");
    }
}
