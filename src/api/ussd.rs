//! Ussd API group (`api/Ussd.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_get_value};

/// Ussd API group.
pub struct Ussd<'a> {
    conn: &'a Connection,
}

impl<'a> Ussd<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Ussd { conn }
    }

    /// `ussd/status`. USSD status.
    pub fn status(&self) -> Result<Value> {
        get_value(self.conn, "api/ussd/status")
    }

    /// `ussd/get`. Current USSD session.
    pub fn get(&self) -> Result<Value> {
        get_value(self.conn, "api/ussd/get")
    }

    /// Send a USSD string.
    pub fn send(&self, content: &str) -> Result<Value> {
        let body: XmlMap = map_of([
            ("content", content.to_string()),
            ("codeType", "codeType".to_string()),
            ("timeout", "".to_string()),
        ]);
        post_get_value(self.conn, "api/ussd/send", &body)
    }
}
