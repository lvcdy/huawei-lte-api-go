//! Ota config group (`config/Ota.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Ota config group.
pub struct OtaConfig<'a> {
    conn: &'a Connection,
}

impl<'a> OtaConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        OtaConfig { conn }
    }

    /// `ota/config.xml`. OTA configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/ota/config.xml")
    }
}
