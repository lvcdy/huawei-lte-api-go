//! Wifi config group (`config/Wifi.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Wifi config group.
pub struct WifiConfig<'a> {
    conn: &'a Connection,
}

impl<'a> WifiConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        WifiConfig { conn }
    }

    /// `wifi/config.xml`. WiFi configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/wifi/config.xml")
    }

    /// `wifi/configure.xml`. WiFi configuration.
    pub fn configure(&self) -> Result<Value> {
        get_value(self.conn, "config/wifi/configure.xml")
    }

    /// `wifi/countryChannel.xml`. WiFi country channel.
    pub fn country_channel(&self) -> Result<Value> {
        get_value(self.conn, "config/wifi/countryChannel.xml")
    }

    /// `wifi/channelAutoMatchHardware.xml`. Channel auto-match hardware status.
    pub fn channel_auto_match_hardware(&self) -> Result<Value> {
        get_value(self.conn, "config/wifi/channelAutoMatchHardware.xml")
    }
}
