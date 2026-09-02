//! DDns API group (`api/DDns.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// DDns API group.
pub struct DDns<'a> {
    conn: &'a Connection,
}

impl<'a> DDns<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        DDns { conn }
    }

    /// `ddns/ddns-list`. List of DDNS configurations.
    pub fn get_ddns_list(&self) -> Result<Value> {
        get_value(self.conn, "api/ddns/ddns-list")
    }

    /// `ddns/status`. DDNS status.
    pub fn get_status(&self) -> Result<Value> {
        get_value(self.conn, "api/ddns/status")
    }

    /// `ddns/serverlist`. List of DDNS servers.
    pub fn serverlist(&self) -> Result<Value> {
        get_value(self.conn, "api/ddns/serverlist")
    }
}
