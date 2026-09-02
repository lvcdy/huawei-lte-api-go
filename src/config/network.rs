//! Network config group (`config/Network.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Network config group.
pub struct NetworkConfig<'a> {
    conn: &'a Connection,
}

impl<'a> NetworkConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        NetworkConfig { conn }
    }

    /// `network/net-mode.xml`. Network mode.
    pub fn net_mode(&self) -> Result<Value> {
        get_value(self.conn, "config/network/net-mode.xml")
    }

    /// `network/networkmode.xml`. Network mode.
    pub fn networkmode(&self) -> Result<Value> {
        get_value(self.conn, "config/network/networkmode.xml")
    }

    /// `network/config.xml`. Network configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/network/config.xml")
    }

    /// `network/networkband_null.xml`. Network band (null).
    pub fn networkband_null(&self) -> Result<Value> {
        get_value(self.conn, "config/network/networkband_null.xml")
    }

    /// `network/setOnly4g.xml`. Set only 4G.
    pub fn set_only_4g(&self) -> Result<Value> {
        get_value(self.conn, "config/network/setOnly4g.xml")
    }
}
