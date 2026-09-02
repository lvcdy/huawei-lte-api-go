//! Lan API group (`api/Lan.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Lan API group.
pub struct Lan<'a> {
    conn: &'a Connection,
}

impl<'a> Lan<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Lan { conn }
    }

    /// `lan/HostInfo`. Connected host information.
    pub fn host_info(&self) -> Result<Value> {
        get_value(self.conn, "api/lan/HostInfo")
    }
}
