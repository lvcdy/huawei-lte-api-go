//! Global config group (`config/Global.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Global config group.
pub struct GlobalConfig<'a> {
    conn: &'a Connection,
}

impl<'a> GlobalConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        GlobalConfig { conn }
    }

    /// `global/languagelist.xml`. Supported language list.
    pub fn languagelist(&self) -> Result<Value> {
        get_value(self.conn, "config/global/languagelist.xml")
    }

    /// `global/config.xml`. Global configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/global/config.xml")
    }

    /// `global/net-type.xml`. Network type.
    pub fn net_type(&self) -> Result<Value> {
        get_value(self.conn, "config/global/net-type.xml")
    }
}
