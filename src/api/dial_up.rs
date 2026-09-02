//! DialUp API group (`api/DialUp.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::{XmlMap, XmlValue};

use super::{get_value, post_set};

/// DialUp API group.
pub struct DialUp<'a> {
    conn: &'a Connection,
}

impl<'a> DialUp<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        DialUp { conn }
    }

    /// `dialup/mobile-dataswitch`. Current LTE modem toggle state.
    pub fn mobile_dataswitch(&self) -> Result<Value> {
        get_value(self.conn, "api/dialup/mobile-dataswitch")
    }

    /// `dialup/connection`. Connection settings.
    pub fn connection(&self) -> Result<Value> {
        get_value(self.conn, "api/dialup/connection")
    }

    /// `dialup/dialup-feature-switch`. Dial-up feature switch status.
    pub fn dialup_feature_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/dialup/dialup-feature-switch")
    }

    /// `dialup/profiles`. Dial-up profiles.
    pub fn profiles(&self) -> Result<Value> {
        get_value(self.conn, "api/dialup/profiles")
    }

    /// `dialup/auto-apn`. Auto APN settings.
    pub fn auto_apn(&self) -> Result<Value> {
        get_value(self.conn, "api/dialup/auto-apn")
    }

    /// Initiate a dial-up connection.
    pub fn dial(&self) -> Result<String> {
        let body: XmlMap = map_of([("Action", "1".to_string())]);
        post_set(self.conn, "api/dialup/dial", &body)
    }

    /// Toggle the LTE modem state.
    ///
    /// * `dataswitch` — `0` to disable, `1` to enable.
    pub fn set_mobile_dataswitch(&self, dataswitch: i64) -> Result<String> {
        let body: XmlMap = map_of([("dataswitch", dataswitch.to_string())]);
        post_set(self.conn, "api/dialup/mobile-dataswitch", &body)
    }

    /// Set the default dial-up profile.
    pub fn set_default_profile(&self, index: i64) -> Result<String> {
        let body: XmlMap = map_of([
            ("SetDefault", index.to_string()),
            ("Delete", "0".to_string()),
            ("Modify", "0".to_string()),
        ]);
        post_set(self.conn, "api/dialup/profiles", &body)
    }

    /// Delete a dial-up profile.
    pub fn delete_profile(&self, index: i64) -> Result<String> {
        let body: XmlMap = map_of([
            ("SetDefault", "0".to_string()),
            ("Delete", index.to_string()),
            ("Modify", "0".to_string()),
        ]);
        post_set(self.conn, "api/dialup/profiles", &body)
    }

    /// Create a new dial-up profile.
    ///
    /// `auth_mode` / `ip_type` are strings mirroring the Python enum `.value`.
    #[allow(clippy::too_many_arguments)]
    pub fn create_profile(
        &self,
        name: &str,
        username: Option<&str>,
        password: Option<&str>,
        apn: Option<&str>,
        dialup_number: Option<&str>,
        auth_mode: &str,
        ip_type: &str,
        is_default: bool,
    ) -> Result<String> {
        let mut profile = XmlMap::new();
        profile.insert("Index".into(), XmlValue::Text("".into()));
        profile.insert("IsValid".into(), XmlValue::Text("1".into()));
        profile.insert("Name".into(), XmlValue::Text(name.to_string()));
        profile.insert(
            "ApnIsStatic".into(),
            XmlValue::Text(if apn.is_some() {
                "1".to_string()
            } else {
                "0".to_string()
            }),
        );
        profile.insert(
            "ApnName".into(),
            XmlValue::Text(apn.unwrap_or("").to_string()),
        );
        profile.insert(
            "DialupNum".into(),
            XmlValue::Text(dialup_number.unwrap_or("").to_string()),
        );
        profile.insert(
            "Username".into(),
            XmlValue::Text(username.unwrap_or("").to_string()),
        );
        profile.insert(
            "Password".into(),
            XmlValue::Text(password.unwrap_or("").to_string()),
        );
        profile.insert("AuthMode".into(), XmlValue::Text(auth_mode.to_string()));
        profile.insert("IpIsStatic".into(), XmlValue::Text("".into()));
        profile.insert("IpAddress".into(), XmlValue::Text("".into()));
        profile.insert("DnsIsStatic".into(), XmlValue::Text("".into()));
        profile.insert("PrimaryDns".into(), XmlValue::Text("".into()));
        profile.insert("SecondaryDns".into(), XmlValue::Text("".into()));
        profile.insert("ReadOnly".into(), XmlValue::Text("0".into()));
        profile.insert("iptype".into(), XmlValue::Text(ip_type.to_string()));

        let mut body = XmlMap::new();
        body.insert(
            "SetDefault".into(),
            XmlValue::Text(if is_default {
                "1".to_string()
            } else {
                "0".to_string()
            }),
        );
        body.insert("Delete".into(), XmlValue::Text("0".into()));
        body.insert("Modify".into(), XmlValue::Text("1".into()));
        body.insert("Profile".into(), XmlValue::Map(profile));
        post_set(self.conn, "api/dialup/profiles", &body)
    }

    /// Update an existing dial-up profile.
    ///
    /// `auth_mode` / `ip_type` are strings mirroring the Python enum `.value`.
    #[allow(clippy::too_many_arguments)]
    pub fn update_profile(
        &self,
        index: i64,
        name: &str,
        username: Option<&str>,
        password: Option<&str>,
        apn: Option<&str>,
        dialup_number: Option<&str>,
        auth_mode: &str,
        ip_type: &str,
        is_default: bool,
    ) -> Result<String> {
        let mut profile = XmlMap::new();
        profile.insert("Index".into(), XmlValue::Text(index.to_string()));
        profile.insert("IsValid".into(), XmlValue::Text("1".into()));
        profile.insert("Name".into(), XmlValue::Text(name.to_string()));
        profile.insert(
            "ApnIsStatic".into(),
            XmlValue::Text(if apn.is_some() {
                "1".to_string()
            } else {
                "0".to_string()
            }),
        );
        profile.insert(
            "ApnName".into(),
            XmlValue::Text(apn.unwrap_or("").to_string()),
        );
        profile.insert(
            "DialupNum".into(),
            XmlValue::Text(dialup_number.unwrap_or("").to_string()),
        );
        profile.insert(
            "Username".into(),
            XmlValue::Text(username.unwrap_or("").to_string()),
        );
        profile.insert(
            "Password".into(),
            XmlValue::Text(password.unwrap_or("").to_string()),
        );
        profile.insert("AuthMode".into(), XmlValue::Text(auth_mode.to_string()));
        profile.insert("IpIsStatic".into(), XmlValue::Text("".into()));
        profile.insert("IpAddress".into(), XmlValue::Text("".into()));
        profile.insert("DnsIsStatic".into(), XmlValue::Text("".into()));
        profile.insert("PrimaryDns".into(), XmlValue::Text("".into()));
        profile.insert("SecondaryDns".into(), XmlValue::Text("".into()));
        profile.insert("ReadOnly".into(), XmlValue::Text("0".into()));
        profile.insert("iptype".into(), XmlValue::Text(ip_type.to_string()));

        let mut body = XmlMap::new();
        body.insert(
            "SetDefault".into(),
            XmlValue::Text(if is_default {
                index.to_string()
            } else {
                "0".to_string()
            }),
        );
        body.insert("Delete".into(), XmlValue::Text("0".into()));
        body.insert("Modify".into(), XmlValue::Text("2".into()));
        body.insert("Profile".into(), XmlValue::Map(profile));
        post_set(self.conn, "api/dialup/profiles", &body)
    }

    /// Set connection settings.
    ///
    /// * `max_idle_time` — auto disconnect interval, `0` is always on.
    #[allow(clippy::too_many_arguments)]
    pub fn set_connection_settings(
        &self,
        roam_auto_connect_enable: bool,
        max_idle_time: i64,
        connect_mode: i64,
        mtu: i64,
        auto_dial_switch: bool,
        pdp_always_on: bool,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            (
                "RoamAutoConnectEnable",
                if roam_auto_connect_enable {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("MaxIdelTime", max_idle_time.to_string()),
            ("ConnectMode", connect_mode.to_string()),
            ("MTU", mtu.to_string()),
            (
                "auto_dial_switch",
                if auto_dial_switch {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            (
                "pdp_always_on",
                if pdp_always_on {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
        ]);
        post_set(self.conn, "api/dialup/connection", &body)
    }
}
