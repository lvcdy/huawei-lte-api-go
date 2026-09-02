//! Global API group (`api/Global.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Global API group.
pub struct Global<'a> {
    conn: &'a Connection,
}

impl<'a> Global<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Global { conn }
    }

    /// `global/module-switch`. Module switch status.
    pub fn module_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/global/module-switch")
    }

    /// `global/storage-getitem`. Storage item (reverse engineered).
    pub fn storage_get_item(&self) -> Result<Value> {
        get_value(self.conn, "api/global/storage-getitem")
    }

    /// `global/custommenu-url`. Custom menu URL.
    pub fn custommenu_url(&self) -> Result<Value> {
        get_value(self.conn, "api/global/custommenu-url")
    }
}
