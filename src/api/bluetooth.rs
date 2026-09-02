//! Bluetooth API group (`api/Bluetooth.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Bluetooth API group.
pub struct Bluetooth<'a> {
    conn: &'a Connection,
}

impl<'a> Bluetooth<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Bluetooth { conn }
    }

    /// `bluetooth/settings`. Bluetooth settings (reverse engineered).
    pub fn settings(&self) -> Result<Value> {
        get_value(self.conn, "api/bluetooth/settings")
    }

    /// `bluetooth/scan`. Scan for Bluetooth devices (reverse engineered).
    pub fn scan(&self) -> Result<Value> {
        get_value(self.conn, "api/bluetooth/scan")
    }
}
