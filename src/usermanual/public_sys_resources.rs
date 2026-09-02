//! Public Sys Resources usermanual group (`usermanual/PublicSysResources.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// PublicSysResources usermanual group.
pub struct PublicSysResources<'a> {
    conn: &'a Connection,
}

impl<'a> PublicSysResources<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        PublicSysResources { conn }
    }

    /// `public_sys-resources/config.xml`. Public system resources configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "usermanual/public_sys-resources/config.xml")
    }
}
