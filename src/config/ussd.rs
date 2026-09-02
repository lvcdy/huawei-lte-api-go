//! Ussd config group (`config/Ussd.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Ussd config group.
pub struct UssdConfig<'a> {
    conn: &'a Connection,
}

impl<'a> UssdConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        UssdConfig { conn }
    }

    /// `ussd/prepaidussd.xml`. Prepaid USSD configuration.
    pub fn prepaidussd(&self) -> Result<Value> {
        get_value(self.conn, "config/ussd/prepaidussd.xml")
    }

    /// `ussd/postpaidussd.xml`. Postpaid USSD configuration.
    pub fn postpaidussd(&self) -> Result<Value> {
        get_value(self.conn, "config/ussd/postpaidussd.xml")
    }
}
