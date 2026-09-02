//! Ota API group (`api/Ota.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Ota API group.
pub struct Ota<'a> {
    conn: &'a Connection,
}

impl<'a> Ota<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Ota { conn }
    }

    /// `ota/status`. OTA update status.
    pub fn status(&self) -> Result<Value> {
        get_value(self.conn, "api/ota/status")
    }
}
