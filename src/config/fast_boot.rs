//! FastBoot config group (`config/FastBoot.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// FastBoot config group.
pub struct FastBootConfig<'a> {
    conn: &'a Connection,
}

impl<'a> FastBootConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        FastBootConfig { conn }
    }

    /// `fastboot/config.xml`. Fast boot configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/fastboot/config.xml")
    }
}
