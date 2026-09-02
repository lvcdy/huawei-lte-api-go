//! Led API group (`api/Led.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Led API group.
pub struct Led<'a> {
    conn: &'a Connection,
}

impl<'a> Led<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Led { conn }
    }

    /// `led/nightmode`. LED night mode status.
    pub fn nightmode(&self) -> Result<Value> {
        get_value(self.conn, "api/led/nightmode")
    }

    /// `led/appctrlled`. LED app control status.
    pub fn appctrlled(&self) -> Result<Value> {
        get_value(self.conn, "api/led/appctrlled")
    }
}
