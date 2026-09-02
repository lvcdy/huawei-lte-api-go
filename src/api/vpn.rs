//! Vpn API group (`api/Vpn.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_set};

/// Vpn API group.
pub struct Vpn<'a> {
    conn: &'a Connection,
}

impl<'a> Vpn<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Vpn { conn }
    }

    /// `vpn/feature-switch`. VPN feature switch status.
    pub fn feature_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/vpn/feature-switch")
    }

    /// `vpn/br_list`. VPN bridge list.
    pub fn br_list(&self) -> Result<Value> {
        get_value(self.conn, "api/vpn/br_list")
    }

    /// `vpn/ipsec_settings`. IPSec settings.
    pub fn ipsec_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/vpn/ipsec_settings")
    }

    /// `vpn/l2tp_settings`. L2TP settings.
    pub fn l2tp_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/vpn/l2tp_settings")
    }

    /// `vpn/pptp_settings`. PPTP settings.
    pub fn pptp_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/vpn/pptp_settings")
    }

    /// Enable/disable a VPN type.
    ///
    /// * `enable` — `true` to enable, `false` to disable.
    /// * `vpn_type` — e.g. `"pptp"` or `"l2tp"` (mirrors the Python `VPNType.value`).
    pub fn toggle_status(&self, enable: bool, vpn_type: &str) -> Result<String> {
        let body: XmlMap = map_of([(
            "enable",
            if enable {
                "1".to_string()
            } else {
                "0".to_string()
            },
        )]);
        post_set(self.conn, &format!("api/vpn/{vpn_type}_settings"), &body)
    }

    /// `vpn/status`. VPN status (reverse engineered).
    pub fn status(&self) -> Result<Value> {
        get_value(self.conn, "api/vpn/status")
    }
}
