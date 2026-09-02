//! Device config group (`config/Device.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Device config group.
pub struct DeviceConfig<'a> {
    conn: &'a Connection,
}

impl<'a> DeviceConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        DeviceConfig { conn }
    }

    /// `device/config.xml`. Device configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/device/config.xml")
    }
}
