//! UsbStorage API group (`api/UsbStorage.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// UsbStorage API group.
pub struct UsbStorage<'a> {
    conn: &'a Connection,
}

impl<'a> UsbStorage<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        UsbStorage { conn }
    }

    /// `usbstorage/fsstatus`. File-system status.
    pub fn fsstatus(&self) -> Result<Value> {
        get_value(self.conn, "api/usbstorage/fsstatus")
    }

    /// `usbstorage/usbaccount`. USB account.
    pub fn usbaccount(&self) -> Result<Value> {
        get_value(self.conn, "api/usbstorage/usbaccount")
    }
}
