//! WebUICfg config group (`config/WebUICfg.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// WebUICfg config group.
pub struct WebUICfgConfig<'a> {
    conn: &'a Connection,
}

impl<'a> WebUICfgConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        WebUICfgConfig { conn }
    }

    /// `webuicfg/config.xml`. Web UI configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/webuicfg/config.xml")
    }
}
