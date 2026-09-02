//! Voice config group (`config/Voice.py`).

use serde_json::Value;

use crate::api::get_value;
use crate::connection::Connection;
use crate::errors::Result;

/// Voice config group.
pub struct VoiceConfig<'a> {
    conn: &'a Connection,
}

impl<'a> VoiceConfig<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        VoiceConfig { conn }
    }

    /// `voice/config.xml`. Voice configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "config/voice/config.xml")
    }

    /// `voice/country.xml`. Voice country.
    pub fn country(&self) -> Result<Value> {
        get_value(self.conn, "config/voice/country.xml")
    }
}
