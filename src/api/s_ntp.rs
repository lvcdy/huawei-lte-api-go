//! SNtp API group (`api/SNtp.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// SNtp API group.
pub struct SNtp<'a> {
    conn: &'a Connection,
}

impl<'a> SNtp<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        SNtp { conn }
    }

    /// `sntp/settings`. SNTP settings.
    pub fn get_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/sntp/settings")
    }

    /// `sntp/sntpswitch`. SNTP switch.
    pub fn sntpswitch(&self) -> Result<Value> {
        get_value(self.conn, "api/sntp/sntpswitch")
    }

    /// `sntp/serverinfo`. SNTP server information.
    pub fn serverinfo(&self) -> Result<Value> {
        get_value(self.conn, "api/sntp/serverinfo")
    }

    /// `sntp/timeinfo`. SNTP time information.
    pub fn timeinfo(&self) -> Result<Value> {
        get_value(self.conn, "api/sntp/timeinfo")
    }
}
