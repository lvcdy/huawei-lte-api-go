//! Statistic API group (`api/Statistic.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Statistic API group.
pub struct Statistic<'a> {
    conn: &'a Connection,
}

impl<'a> Statistic<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Statistic { conn }
    }

    /// `statistic/feature-roam-statistic`. Roam statistic feature status.
    pub fn feature_roam_statistic(&self) -> Result<Value> {
        get_value(self.conn, "api/statistic/feature-roam-statistic")
    }
}
