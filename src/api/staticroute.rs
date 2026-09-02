//! Staticroute API group (`api/Staticroute.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Staticroute API group.
pub struct Staticroute<'a> {
    conn: &'a Connection,
}

impl<'a> Staticroute<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Staticroute { conn }
    }

    /// `staticroute/wanpath`. WAN path.
    pub fn wanpath(&self) -> Result<Value> {
        get_value(self.conn, "api/staticroute/wanpath")
    }
}
