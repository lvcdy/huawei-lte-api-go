//! Time API group (`api/Time.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Time API group.
pub struct Time<'a> {
    conn: &'a Connection,
}

impl<'a> Time<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Time { conn }
    }

    /// `time/timeout`. Current time timeout.
    pub fn timeout(&self) -> Result<Value> {
        get_value(self.conn, "api/time/timeout")
    }
}
