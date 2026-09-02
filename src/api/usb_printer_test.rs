//! Tests for the [`UsbPrinter`] API group.

use crate::api::usb_printer::UsbPrinter;
use crate::testsupport::conn_with;

/// `usbprinter/printerlist` resolves.
#[test]
fn printerlist_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/usbprinter/printerlist",
        "<response><PrinterList><Printer><Name>hp</Name></Printer></PrinterList></response>",
    );

    let up = UsbPrinter::new(&conn);
    let value = up.printerlist().expect("ok");
    assert_eq!(value["PrinterList"]["Printer"]["Name"], "hp");
}
