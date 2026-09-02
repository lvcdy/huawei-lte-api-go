//! Developer API group (`api/Developer.py`).

use serde_json::Value;
use std::collections::BTreeMap;

use crate::connection::Connection;
use crate::errors::Result;
use crate::xml::{XmlMap, XmlValue};

use super::{get_value, post_set};

/// Developer API group.
pub struct Developer<'a> {
    conn: &'a Connection,
}

impl<'a> Developer<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Developer { conn }
    }

    /// `developer/developermode-featureswitch`. Developer mode feature switch status.
    pub fn developermode_featureswitch(&self) -> Result<Value> {
        get_value(self.conn, "api/developer/developermode-featureswitch")
    }

    /// `app/atport-status`. AT port status.
    pub fn atport_status(&self) -> Result<Value> {
        get_value(self.conn, "api/app/atport-status")
    }

    /// `developer/atport-status` (POST). Enable/disable the AT/telnet debug
    /// port. Requires developer-mode authentication on the device.
    ///
    /// * `enable` — `true` to request `<enable>1</enable>`.
    ///
    /// Supplementary endpoint from
    /// Brovi-Huawei-5G-CPE-Manager.
    pub fn set_atport_status(&self, enable: bool) -> Result<String> {
        let mut body: XmlMap = BTreeMap::new();
        body.insert(
            "enable".to_string(),
            XmlValue::Text(if enable {
                "1".to_string()
            } else {
                "0".to_string()
            }),
        );
        post_set(self.conn, "api/developer/atport-status", &body)
    }
}
