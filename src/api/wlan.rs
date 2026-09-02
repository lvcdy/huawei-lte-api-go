//! WLan API group (`api/WLan.py`).
//!
//! Wi-Fi settings: basic/security config, WPS, MAC filtering and guest
//! networks.

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::{XmlMap, XmlValue};

use super::{get_value, post_set};

/// WLan API group.
pub struct WLan<'a> {
    conn: &'a Connection,
}

impl<'a> WLan<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        WLan { conn }
    }

    /// `wlan/wifi-feature-switch`. Wi-Fi feature switch.
    pub fn wifi_feature_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wifi-feature-switch")
    }

    /// `wlan/station-information`. Connected-station information.
    pub fn station_information(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/station-information")
    }

    /// `wlan/basic-settings`. Basic Wi-Fi settings.
    pub fn basic_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/basic-settings")
    }

    /// Update the basic Wi-Fi settings (SSID, hidden, restart).
    pub fn set_basic_settings(&self, ssid: &str, hide: bool, wifi_restart: bool) -> Result<String> {
        let body: XmlMap = map_of([
            ("WifiSsid", ssid.to_string()),
            (
                "WifiHide",
                if hide {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            (
                "WifiRestart",
                if wifi_restart {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
        ]);
        post_set(self.conn, "api/wlan/basic-settings", &body)
    }

    /// `wlan/security-settings`. Current Wi-Fi security settings.
    pub fn security_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/security-settings")
    }

    /// Update the Wi-Fi security settings.
    ///
    /// * `wpa_psk` — WPA passphrase.
    /// * `wep_key` — WEP key (`""` when not used).
    /// * `wpa_encryption_mode` — WPA encryption mode enum as `i64`.
    /// * `wep_encryption_mode` — WEP encryption mode enum as `i64`.
    /// * `auth_mode` — auth mode enum as `i64`.
    /// * `wifi_restart` — restart Wi-Fi radios after applying.
    pub fn set_security_settings(
        &self,
        wpa_psk: &str,
        wep_key: &str,
        wpa_encryption_mode: i64,
        wep_encryption_mode: i64,
        auth_mode: i64,
        wifi_restart: bool,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            ("WifiAuthmode", auth_mode.to_string()),
            ("WifiWepKey1", wep_key.to_string()),
            ("WifiWpaencryptionmodes", wpa_encryption_mode.to_string()),
            ("WifiBasicencryptionmodes", wep_encryption_mode.to_string()),
            ("WifiWpapsk", wpa_psk.to_string()),
            (
                "WifiRestart",
                if wifi_restart {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
        ]);
        post_set(self.conn, "api/wlan/security-settings", &body)
    }

    /// `wlan/multi-security-settings`. Per-SSID security settings.
    pub fn multi_security_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/multi-security-settings")
    }

    /// `wlan/multi-security-settings-ex`. Extended per-SSID security settings.
    pub fn multi_security_settings_ex(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/multi-security-settings-ex")
    }

    /// `wlan/multi-basic-settings`. Per-SSID basic settings.
    pub fn multi_basic_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/multi-basic-settings")
    }

    /// Update basic settings for multiple SSIDs at once.
    ///
    /// * `clients` — list of dicts with format
    ///   `{'wifihostname': hostname, 'WifiMacFilterMac': mac}`.
    pub fn set_multi_basic_settings(&self, clients: &[XmlMap]) -> Result<String> {
        let mut ssids = XmlMap::new();
        ssids.insert(
            "Ssid".to_string(),
            XmlValue::List(clients.iter().map(|c| XmlValue::Map(c.clone())).collect()),
        );
        let mut body = XmlMap::new();
        body.insert("Ssids".to_string(), XmlValue::Map(ssids));
        body.insert("WifiRestart".to_string(), XmlValue::Text("1".to_string()));
        post_set(self.conn, "api/wlan/multi-basic-settings", &body)
    }

    /// `wlan/host-list`. List of associated hosts.
    pub fn host_list(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/host-list")
    }

    /// `wlan/handover-setting`. Handover mode.
    ///
    /// `0` = G3 prefer, `2` = WiFi prefer.
    pub fn handover_setting(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/handover-setting")
    }

    /// `wlan/handover-setting`. Set the handover mode.
    pub fn set_handover_setting(&self, handover: i64) -> Result<String> {
        let body: XmlMap = map_of([("Handover", handover.to_string())]);
        post_set(self.conn, "api/wlan/handover-setting", &body)
    }

    /// `wlan/multi-switch-settings`. Per-SSID switch settings.
    pub fn multi_switch_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/multi-switch-settings")
    }

    /// `wlan/multi-macfilter-settings`. Per-SSID MAC filter settings.
    pub fn multi_macfilter_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/multi-macfilter-settings")
    }

    /// Update MAC filter rules for multiple SSIDs.
    ///
    /// * `clients` — list of dicts (`WifiMacFilterMac0`, `wifihostname0`,
    ///   `Index`, `WifiMacFilterStatus`).
    pub fn set_multi_macfilter_settings(&self, clients: &[XmlMap]) -> Result<String> {
        let mut ssids = XmlMap::new();
        ssids.insert(
            "Ssid".to_string(),
            XmlValue::List(clients.iter().map(|c| XmlValue::Map(c.clone())).collect()),
        );
        let mut body = XmlMap::new();
        body.insert("Ssids".to_string(), XmlValue::Map(ssids));
        post_set(self.conn, "api/wlan/multi-macfilter-settings", &body)
    }

    /// `wlan/multi-macfilter-settings-ex`. Extended MAC filter settings.
    pub fn multi_macfilter_settings_ex(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/multi-macfilter-settings-ex")
    }

    /// `wlan/mac-filter`. MAC filter status.
    pub fn mac_filter(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/mac-filter")
    }

    /// Add a single MAC filter entry.
    pub fn set_mac_filter(&self, hostname: &str, mac: &str) -> Result<String> {
        let body: XmlMap = map_of([
            ("wifihostname", hostname.to_string()),
            ("WifiMacFilterMac", mac.to_string()),
        ]);
        post_set(self.conn, "api/wlan/mac-filter", &body)
    }

    /// `wlan/oled-showpassword`. OLED show-password state.
    pub fn oled_showpassword(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/oled-showpassword")
    }

    /// `wlan/wps`. WPS state.
    pub fn wps(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wps")
    }

    /// `wlan/wps-appin`. WPS appin state.
    pub fn wps_appin(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wps-appin")
    }

    /// Start WPS appin (PIN) pairing.
    pub fn set_wps_appin(&self, wpsappintype: i64, wpsappin: Option<i64>) -> Result<String> {
        let appin = match wpsappin {
            Some(v) => v.to_string(),
            None => String::new(),
        };
        let body: XmlMap = map_of([
            ("wpsappintype", wpsappintype.to_string()),
            ("wpsappin", appin),
        ]);
        post_set(self.conn, "api/wlan/wps-appin", &body)
    }

    /// `wlan/wps-pbc`. WPS push-button state.
    pub fn wps_pbc(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wps-pbc")
    }

    /// Start WPS push-button pairing.
    pub fn set_wps_pbc(&self, wpsmode: i64, ssidindex: i64) -> Result<String> {
        let body: XmlMap = map_of([
            ("WPSMode", wpsmode.to_string()),
            ("ssidindex", ssidindex.to_string()),
        ]);
        post_set(self.conn, "api/wlan/wps-pbc", &body)
    }

    /// `wlan/wps-switch`. WPS switch state.
    pub fn wps_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wps-switch")
    }

    /// Toggle WPS appin.
    pub fn set_wps_switch(&self, appinenable: i64) -> Result<String> {
        let body: XmlMap = map_of([("appinenable", appinenable.to_string())]);
        post_set(self.conn, "api/wlan/wps-switch", &body)
    }

    /// `wlan/status-switch-settings`. Status switch settings.
    pub fn status_switch_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/status-switch-settings")
    }

    /// `wlan/wifiprofile` (reverse engineered, likely unused).
    pub fn wifiprofile(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wifiprofile")
    }

    /// `wlan/wififrequence` (reverse engineered, likely unused).
    pub fn wififrequence(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wififrequence")
    }

    /// `wlan/wifiscanresult` (reverse engineered, likely unused).
    pub fn wifiscanresult(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wifiscanresult")
    }

    /// `wlan/wlandbho` (reverse engineered).
    pub fn wlandbho(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wlandbho")
    }

    /// `wlan/wlan-guide-settings`. Initial setup guide settings.
    pub fn wlan_guide_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wlan-guide-settings")
    }

    /// `wlan/wlanintelligent` (reverse engineered).
    pub fn wlanintelligent(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wlanintelligent")
    }

    /// `wlan/guesttime-setting`. Guest-network time budget.
    pub fn guesttime_setting(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/guesttime-setting")
    }

    /// `wlan/wlan-debug`. WLAN debug/module state.
    ///
    /// Supplementary endpoint from
    /// Brovi-Huawei-5G-CPE-Manager.
    pub fn get_wlan_debug(&self) -> Result<Value> {
        get_value(self.conn, "api/wlan/wlan-debug")
    }

    /// `wlan/wlan-debug` (POST). Set WLAN debug fields.
    ///
    /// The device exposes arbitrary writable keys here; pass any desired
    /// fields (e.g. telnet / developer switches) verbatim.
    ///
    /// Supplementary endpoint from
    /// Brovi-Huawei-5G-CPE-Manager.
    pub fn set_wlan_debug(&self, params: &XmlMap) -> Result<String> {
        post_set(self.conn, "api/wlan/wlan-debug", params)
    }
}
