//! WebSd config group (`config/WebSd.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// WebSd config group.
pub struct WebSdConfig<'a> {
    conn: &'a Connection,
}

impl<'a> WebSdConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        WebSdConfig { conn }
    }

    /// `websd/config.xml`. Web SD configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/websd/config.xml")
    }
}
