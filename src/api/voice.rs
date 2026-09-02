//! Voice API group (`api/Voice.py`).
//!
//! VoIP accounts, codecs and call features.

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// Voice API group.
pub struct Voice<'a> {
    conn: &'a Connection,
}

impl<'a> Voice<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Voice { conn }
    }

    /// `voice/featureswitch`. Voice feature switch.
    pub fn featureswitch(&self) -> Result<Value> {
        get_value(self.conn, "api/voice/featureswitch")
    }

    /// `voice/sipaccount`. SIP account settings.
    pub fn sipaccount(&self) -> Result<Value> {
        get_value(self.conn, "api/voice/sipaccount")
    }

    /// `voice/sipadvance`. Advanced SIP settings.
    pub fn sipadvance(&self) -> Result<Value> {
        get_value(self.conn, "api/voice/sipadvance")
    }

    /// `voice/sipserver`. SIP server settings.
    pub fn sipserver(&self) -> Result<Value> {
        get_value(self.conn, "api/voice/sipserver")
    }

    /// `voice/speeddial`. Speed-dial settings.
    pub fn speeddial(&self) -> Result<Value> {
        get_value(self.conn, "api/voice/speeddial")
    }

    /// `voice/functioncode`. Function code settings.
    pub fn functioncode(&self) -> Result<Value> {
        get_value(self.conn, "api/voice/functioncode")
    }

    /// `voice/voiceadvance`. Advanced voice settings.
    pub fn voiceadvance(&self) -> Result<Value> {
        get_value(self.conn, "api/voice/voiceadvance")
    }

    /// `voice/voicebusy`. Busy handling settings.
    pub fn voicebusy(&self) -> Result<Value> {
        get_value(self.conn, "api/voice/voicebusy")
    }

    /// `voice/codec` (reverse engineered, likely unused).
    pub fn codec(&self) -> Result<Value> {
        get_value(self.conn, "api/voice/codec")
    }

    /// `voice/voiperstatus`. VoIP error status.
    pub fn voiperstatus(&self) -> Result<Value> {
        get_value(self.conn, "api/voice/voiperstatus")
    }

    /// `voice/volte`. VoLTE state.
    pub fn volte(&self) -> Result<Value> {
        get_value(self.conn, "api/voice/volte")
    }
}
