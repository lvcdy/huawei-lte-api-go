//! Pb config group (`config/Pb.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Pb config group.
pub struct PbConfig<'a> {
    conn: &'a Connection,
}

impl<'a> PbConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        PbConfig { conn }
    }

    /// `pb/config.xml`. Phone book configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/pb/config.xml")
    }
}
