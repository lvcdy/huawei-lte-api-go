//! Log API group (`api/Log.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Log API group.
pub struct Log<'a> {
    conn: &'a Connection,
}

impl<'a> Log<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Log { conn }
    }

    /// `log/loginfo`. Log information.
    pub fn loginfo(&self) -> Result<Value> {
        get_value(self.conn, "api/log/loginfo")
    }
}
