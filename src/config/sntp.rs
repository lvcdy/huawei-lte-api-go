//! Sntp config group (`config/Sntp.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Sntp config group.
pub struct SntpConfig<'a> {
    conn: &'a Connection,
}

impl<'a> SntpConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        SntpConfig { conn }
    }

    /// `sntp/config.xml`. SNTP configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/sntp/config.xml")
    }
}
