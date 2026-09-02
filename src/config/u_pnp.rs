//! UPnp config group (`config/UPnp.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// UPnP config group.
pub struct UPnPConfig<'a> {
    conn: &'a Connection,
}

impl<'a> UPnPConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        UPnPConfig { conn }
    }

    /// `upnp/config.xml`. UPnP configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/upnp/config.xml")
    }
}
