//! Lan config group (`config/Lan.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Lan config group.
pub struct LanConfig<'a> {
    conn: &'a Connection,
}

impl<'a> LanConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        LanConfig { conn }
    }

    /// `lan/config.xml`. LAN configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/lan/config.xml")
    }
}
