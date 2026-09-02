//! IPv6 config group (`config/IPv6.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// IPv6 config group.
pub struct Ipv6Config<'a> {
    conn: &'a Connection,
}

impl<'a> Ipv6Config<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Ipv6Config { conn }
    }

    /// `ipv6/config.xml`. IPv6 configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/ipv6/config.xml")
    }
}
