//! DialUp config group (`config/DialUp.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// DialUp config group.
pub struct DialUpConfig<'a> {
    conn: &'a Connection,
}

impl<'a> DialUpConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        DialUpConfig { conn }
    }

    /// `dialup/config.xml`. Dial-up configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/dialup/config.xml")
    }

    /// `dialup/connectmode.xml`. Dial-up connect mode.
    pub fn connectmode(&self) -> Result<Value> {
        get_value(self.conn, "config/dialup/connectmode.xml")
    }

    /// `dialup/profileswitch.xml`. Dial-up profile switch.
    pub fn profileswitch(&self) -> Result<Value> {
        get_value(self.conn, "config/dialup/profileswitch.xml")
    }

    /// `dialup/lmt_auto_mode_disconnect.xml`. Auto-mode disconnect status.
    pub fn lmt_auto_mode_disconnect(&self) -> Result<Value> {
        get_value(self.conn, "config/dialup/lmt_auto_mode_disconnect.xml")
    }
}
