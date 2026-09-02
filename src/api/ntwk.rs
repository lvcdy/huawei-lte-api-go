//! Ntwk API group (`api/Ntwk.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Ntwk API group.
pub struct Ntwk<'a> {
    conn: &'a Connection,
}

impl<'a> Ntwk<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Ntwk { conn }
    }

    /// `ntwk/lan_upnp_portmapping`. LAN UPnP port mapping.
    pub fn lan_upnp_portmapping(&self) -> Result<Value> {
        get_value(self.conn, "api/ntwk/lan_upnp_portmapping")
    }

    /// `ntwk/celllock`. Cell lock status.
    pub fn celllock(&self) -> Result<Value> {
        get_value(self.conn, "api/ntwk/celllock")
    }

    /// `ntwk/dualwaninfo`. Dual WAN information.
    pub fn dualwaninfo(&self) -> Result<Value> {
        get_value(self.conn, "api/ntwk/dualwaninfo")
    }

    /// `ntwk/lan-wan-config`. LAN/WAN configuration.
    pub fn lan_wan_config(&self) -> Result<Value> {
        get_value(self.conn, "api/ntwk/lan-wan-config")
    }
}
