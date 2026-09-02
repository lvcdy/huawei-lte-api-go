//! PcAssistant config group (`config/PcAssistant.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// PcAssistant config group.
pub struct PcAssistantConfig<'a> {
    conn: &'a Connection,
}

impl<'a> PcAssistantConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        PcAssistantConfig { conn }
    }

    /// `pcassistant/config.xml`. PC assistant configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/pcassistant/config.xml")
    }

    /// `pcassistant/updateautorun.xml`. PC assistant auto-run update status.
    pub fn updateautorun(&self) -> Result<Value> {
        get_value(self.conn, "config/pcassistant/updateautorun.xml")
    }
}
