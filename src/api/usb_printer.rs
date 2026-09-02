//! UsbPrinter API group (`api/UsbPrinter.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// UsbPrinter API group.
pub struct UsbPrinter<'a> {
    conn: &'a Connection,
}

impl<'a> UsbPrinter<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        UsbPrinter { conn }
    }

    /// `usbprinter/printerlist`. Connected printers.
    pub fn printerlist(&self) -> Result<Value> {
        get_value(self.conn, "api/usbprinter/printerlist")
    }
}
