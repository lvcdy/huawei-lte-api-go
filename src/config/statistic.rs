//! Statistic config group (`config/Statistic.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Statistic config group.
pub struct StatisticConfig<'a> {
    conn: &'a Connection,
}

impl<'a> StatisticConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        StatisticConfig { conn }
    }

    /// `statistic/config.xml`. Statistic configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/statistic/config.xml")
    }
}
