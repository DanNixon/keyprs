use escpos::{
    driver::Driver,
    errors::Result,
    printer::Printer,
    utils::{RealTimeStatusRequest, RealTimeStatusResponse},
};

pub fn check_online<D: Driver>(driver: &D, printer: &mut Printer<D>) -> Result<bool> {
    printer
        .real_time_status(RealTimeStatusRequest::Printer)?
        .send_status()?;

    let mut buf = [0; 1];
    driver.read(&mut buf)?;

    // Exit if the printer does not report to be online
    let status = RealTimeStatusResponse::parse(RealTimeStatusRequest::Printer, buf[0])?;
    Ok(*status
        .get(&RealTimeStatusResponse::Online)
        .unwrap_or(&false))
}
