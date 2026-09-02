//! Cradle API group (`api/Cradle.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Cradle API group.
pub struct Cradle<'a> {
    conn: &'a Connection,
}

impl<'a> Cradle<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Cradle { conn }
    }

    /// `cradle/status-info`. Cradle status information.
    pub fn status_info(&self) -> Result<Value> {
        get_value(self.conn, "api/cradle/status-info")
    }

    /// `cradle/feature-switch`. Cradle feature switch status.
    pub fn feature_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/cradle/feature-switch")
    }

    /// `cradle/basic-info`. Cradle basic information.
    pub fn basic_info(&self) -> Result<Value> {
        get_value(self.conn, "api/cradle/basic-info")
    }

    /// `cradle/factory-mac`. Cradle factory MAC address.
    pub fn factory_mac(&self) -> Result<Value> {
        get_value(self.conn, "api/cradle/factory-mac")
    }

    /// `cradle/mac-info`. Cradle MAC address information.
    pub fn mac_info(&self) -> Result<Value> {
        get_value(self.conn, "api/cradle/mac-info")
    }
}
