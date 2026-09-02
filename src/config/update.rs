//! Update config group (`config/Update.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Update config group.
pub struct UpdateConfig<'a> {
    conn: &'a Connection,
}

impl<'a> UpdateConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        UpdateConfig { conn }
    }

    /// `update/config.xml`. Update configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/update/config.xml")
    }
}
