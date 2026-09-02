//! TimeRule API group (`api/TimeRule.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// TimeRule API group.
pub struct TimeRule<'a> {
    conn: &'a Connection,
}

impl<'a> TimeRule<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        TimeRule { conn }
    }

    /// `timerule/timerule`. Time rules.
    pub fn timerule(&self) -> Result<Value> {
        get_value(self.conn, "api/timerule/timerule")
    }
}
