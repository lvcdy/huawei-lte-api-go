//! Cwmp API group (`api/Cwmp.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Cwmp API group.
pub struct Cwmp<'a> {
    conn: &'a Connection,
}

impl<'a> Cwmp<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Cwmp { conn }
    }

    /// `cwmp/basic-info`. Basic Cwmp information.
    pub fn basic_info(&self) -> Result<Value> {
        get_value(self.conn, "api/cwmp/basic-info")
    }
}
