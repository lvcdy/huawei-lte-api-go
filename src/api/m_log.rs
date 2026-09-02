//! MLog API group (`api/MLog.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// MLog API group.
pub struct MLog<'a> {
    conn: &'a Connection,
}

impl<'a> MLog<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        MLog { conn }
    }

    /// `mlog/mobile-logger`. Mobile logger information (reverse engineered).
    pub fn mobile_logger(&self) -> Result<Value> {
        get_value(self.conn, "api/mlog/mobile-logger")
    }
}
