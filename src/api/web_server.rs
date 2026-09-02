//! WebServer API group (`api/WebServer.py`).
//!
//! Session/token endpoints that back the connection layer.

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// WebServer API group.
pub struct WebServer<'a> {
    conn: &'a Connection,
}

impl<'a> WebServer<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        WebServer { conn }
    }

    /// `webserver/publickey`. Public key for login encryption.
    pub fn publickey(&self) -> Result<Value> {
        get_value(self.conn, "api/webserver/publickey")
    }

    /// `webserver/token`. Session token.
    pub fn token(&self) -> Result<Value> {
        get_value(self.conn, "api/webserver/token")
    }

    /// `webserver/white_list_switch`. White-list switch.
    pub fn white_list_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/webserver/white_list_switch")
    }

    /// `webserver/SesTokInfo`. Session-token info (valid for e.g. B310s-22).
    pub fn ses_tok_info(&self) -> Result<Value> {
        get_value(self.conn, "api/webserver/SesTokInfo")
    }
}
