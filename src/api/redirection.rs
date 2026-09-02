//! Redirection API group (`api/Redirection.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Redirection API group.
pub struct Redirection<'a> {
    conn: &'a Connection,
}

impl<'a> Redirection<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Redirection { conn }
    }

    /// `redirection/homepage`. Redirection homepage.
    pub fn homepage(&self) -> Result<Value> {
        get_value(self.conn, "api/redirection/homepage")
    }
}
