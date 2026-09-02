//! Syslog API group (`api/Syslog.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_set};

/// Syslog API group.
pub struct Syslog<'a> {
    conn: &'a Connection,
}

impl<'a> Syslog<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Syslog { conn }
    }

    /// `syslog/querylog`. Query the syslog.
    pub fn querylog(&self) -> Result<Value> {
        get_value(self.conn, "api/syslog/querylog")
    }

    /// Clear the syslog (process-log `command=clear`).
    pub fn clear(&self) -> Result<String> {
        let body: XmlMap = map_of([("command", "clear".to_string())]);
        post_set(self.conn, "api/syslog/processlog", &body)
    }
}
