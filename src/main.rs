mod output;
mod printer;

use crate::output::{OutputMode, PrinterExt};
use clap::{Args, Parser};
use escpos::{
    driver::{ConsoleDriver, Driver, SerialPortDriver},
    printer::Printer,
    printer_options::PrinterOptions,
    utils::{JustifyMode, Protocol},
};
use miette::{IntoDiagnostic, Result, miette};
use std::{io::Read, time::Duration};

/// A very barebones tool to backup secrets to paper.
///
/// Takes secret text (e.g. passwords, keys, textual treasure maps) and prints them using a thermal/POS printer.
/// Adds a timestamp and an optional note/identifier for easy discovery at the appropriate time.
///
/// Provides the option of outputting any combination of the following:
/// the raw text,
/// a QR code,
/// or
/// a friendly representation of the input.
///
/// Friendly representations change the formatting of certain types of secrets to assist manual reproduction when required.
/// The following have friendly representations, requesting a friendly representation for anything else will raise an error:
/// age secret keys
/// and
/// anything in PGP ASCII armor format
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// An optional node to add to the printout, can be used as an identifier
    #[arg(short, long)]
    note: Option<String>,

    /// The output format(s) to print the input as
    #[arg(short, long = "format")]
    formats: Vec<OutputMode>,

    /// Number of columns the printer has, used to format the output correctly
    #[arg(long, default_value = "42", value_parser = clap::value_parser!(u8).range(8..))]
    printer_columns: u8,

    #[clap(flatten)]
    printer: SerialPrinterOptions,
}

#[derive(Debug, Args)]
struct SerialPrinterOptions {
    /// The serial port that the printer is connected to
    #[arg(long)]
    serial_port: Option<String>,

    /// The baud rate the printer is expecting serial communications at
    #[arg(long, default_value = "38400")]
    serial_baud: u32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.formats.is_empty() {
        return Err(miette::miette!("No output formats specified"));
    }

    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .into_diagnostic()?;

    let options = {
        let mut options = PrinterOptions::default();
        options.characters_per_line(cli.printer_columns);
        Some(options)
    };

    if let Some(port) = cli.printer.serial_port {
        let driver =
            SerialPortDriver::open(&port, cli.printer.serial_baud, Some(Duration::from_secs(5)))
                .into_diagnostic()?;
        let mut printer = Printer::new(driver.clone(), Protocol::default(), options);
        init_and_check_printer(&driver, &mut printer)?;
        do_print(&mut printer, cli.formats, cli.note, input)
    } else {
        let driver = ConsoleDriver::open(true);
        let mut printer = Printer::new(driver, Protocol::default(), options);
        do_print(&mut printer, cli.formats, cli.note, input)
    }
}

fn do_print(
    printer: &mut Printer<impl Driver>,
    formats: Vec<OutputMode>,
    note: Option<String>,
    input: String,
) -> miette::Result<()> {
    println!("📃 Printing...");

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z");

    printer
        .reset_size()
        .into_diagnostic()?
        .justify(JustifyMode::CENTER)
        .into_diagnostic()?
        .writeln("=== Keyprs Printout ===")
        .into_diagnostic()?
        .justify(JustifyMode::LEFT)
        .into_diagnostic()?
        .print()
        .into_diagnostic()?
        .writeln("github.com/DanNixon/keyprs")
        .into_diagnostic()?
        .writeln(&format!("Git rev.: {}", git_version::git_version!()))
        .into_diagnostic()?
        .writeln(&format!("Printed: {timestamp}"))
        .into_diagnostic()?
        .print()
        .into_diagnostic()?;

    if let Some(note) = note {
        printer
            .writeln(&format!("Note: {note}"))
            .into_diagnostic()?
            .print()
            .into_diagnostic()?;
    }

    printer
        .feed()
        .into_diagnostic()?
        .print()
        .into_diagnostic()?;

    for f in formats {
        printer.print_secret(f, &input)?;
        printer
            .feed()
            .into_diagnostic()?
            .print()
            .into_diagnostic()?;
    }

    printer
        .justify(JustifyMode::CENTER)
        .into_diagnostic()?
        .writeln("=== End ===")
        .into_diagnostic()?
        .feed()
        .into_diagnostic()?
        .print_cut()
        .into_diagnostic()?;

    println!("✅ Done!");
    println!("👀 Be sure to verify the printed output is correct and readable!");

    Ok(())
}

fn init_and_check_printer<D: Driver>(driver: &D, printer: &mut Printer<D>) -> Result<()> {
    printer.init().into_diagnostic()?;

    println!("🔍 Checking printer is ready...");
    if printer::check_online(driver, printer).into_diagnostic()? {
        println!("✅ Printer is online.");
        Ok(())
    } else {
        Err(miette!("Printer is offline"))
    }
}
