//! Stk config group (`config/Stk.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Stk config group.
pub struct StkConfig<'a> {
    conn: &'a Connection,
}

impl<'a> StkConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        StkConfig { conn }
    }

    /// `stk/config.xml`. STK configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/stk/config.xml")
    }
}
