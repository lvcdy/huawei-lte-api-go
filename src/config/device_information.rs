//! DeviceInformation config group (`config/DeviceInformation.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// DeviceInformation config group.
pub struct DeviceInformationConfig<'a> {
    conn: &'a Connection,
}

impl<'a> DeviceInformationConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        DeviceInformationConfig { conn }
    }

    /// `deviceinformation/config.xml`. Device information configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/deviceinformation/config.xml")
    }
}
