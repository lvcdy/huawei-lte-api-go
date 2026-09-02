//! Developermode API group.
//!
//! Supplementary group ported from
//! Brovi-Huawei-5G-CPE-Manager: developer-mode feature switches on
//! newer 5G CPE devices.

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Developermode API group.
pub struct Developermode<'a> {
    conn: &'a Connection,
}

impl<'a> Developermode<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Developermode { conn }
    }

    /// `developermode/developer-mode`. Developer-mode switch status.
    pub fn developer_mode(&self) -> Result<Value> {
        get_value(self.conn, "api/developermode/developer-mode")
    }

    /// `developermode/developer-item`. Developer-mode sub-item status.
    pub fn developer_item(&self) -> Result<Value> {
        get_value(self.conn, "api/developermode/developer-item")
    }
}
