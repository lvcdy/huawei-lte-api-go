//! Firewall config group (`config/Firewall.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Firewall config group.
pub struct FirewallConfig<'a> {
    conn: &'a Connection,
}

impl<'a> FirewallConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        FirewallConfig { conn }
    }

    /// `firewall/config.xml`. Firewall configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/firewall/config.xml")
    }
}
