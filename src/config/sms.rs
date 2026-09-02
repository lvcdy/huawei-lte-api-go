//! Sms config group (`config/Sms.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Sms config group.
pub struct SmsConfig<'a> {
    conn: &'a Connection,
}

impl<'a> SmsConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        SmsConfig { conn }
    }

    /// `sms/config.xml`. SMS configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/sms/config.xml")
    }
}
