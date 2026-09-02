//! Pincode config group (`config/Pincode.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Pincode config group.
pub struct PincodeConfig<'a> {
    conn: &'a Connection,
}

impl<'a> PincodeConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        PincodeConfig { conn }
    }

    /// `pincode/config.xml`. Pin code configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/pincode/config.xml")
    }
}
