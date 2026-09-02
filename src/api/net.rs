//! Net API group (`api/Net.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::enums::net::NetworkMode;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_set};

/// Net API group.
pub struct Net<'a> {
    conn: &'a Connection,
}

impl<'a> Net<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Net { conn }
    }

    /// `net/current-plmn`. Current Public Land Mobile Network.
    pub fn current_plmn(&self) -> Result<Value> {
        get_value(self.conn, "api/net/current-plmn")
    }

    /// `net/net-mode`. Current network mode.
    pub fn net_mode(&self) -> Result<Value> {
        get_value(self.conn, "api/net/net-mode")
    }

    /// Set the network mode and bands.
    ///
    /// * `lteband` — LTE band bitmask (`LTEBand::All` for all / non-4G).
    /// * `networkband` — 3G band bitmask (`NetworkBand::All` for all / non-3G).
    /// * `networkmode` — desired [`NetworkMode`].
    pub fn set_net_mode(
        &self,
        lteband: u64,
        networkband: u64,
        networkmode: NetworkMode,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            ("NetworkMode", networkmode.as_str().to_string()),
            ("NetworkBand", format!("{networkband:x}")),
            ("LTEBand", format!("{lteband:x}")),
        ]);
        post_set(self.conn, "api/net/net-mode", &body)
    }

    /// `net/network`. Network information.
    pub fn network(&self) -> Result<Value> {
        get_value(self.conn, "api/net/network")
    }

    /// Set the network mode and band (different value ranges than
    /// [`Net::set_net_mode`]).
    pub fn set_network(&self, networkmode: &str, networkband: &str) -> Result<String> {
        let body: XmlMap = map_of([
            ("NetworkMode", networkmode.to_string()),
            ("NetworkBand", networkband.to_string()),
        ]);
        post_set(self.conn, "api/net/network", &body)
    }

    /// `net/register`. Network registration status.
    pub fn register(&self) -> Result<Value> {
        get_value(self.conn, "api/net/register")
    }

    /// Set the network registration (manual/auto selection).
    ///
    /// * `mode` — `"1"` manual, `"0"` auto.
    /// * `plmn` — PLMN code, `""` for auto.
    /// * `rat` — `"0"` 2G, `"2"` 3G, `"7"` 4G; `""` for auto.
    pub fn set_register(&self, mode: &str, plmn: &str, rat: &str) -> Result<String> {
        let body: XmlMap = map_of([
            ("Mode", mode.to_string()),
            ("Plmn", plmn.to_string()),
            ("Rat", rat.to_string()),
        ]);
        post_set(self.conn, "api/net/register", &body)
    }

    /// `net/net-mode-list`. Available network modes.
    pub fn net_mode_list(&self) -> Result<Value> {
        get_value(self.conn, "api/net/net-mode-list")
    }

    /// `net/plmn-list`. Available PLMNs.
    pub fn plmn_list(&self) -> Result<Value> {
        get_value(self.conn, "api/net/plmn-list")
    }

    /// `net/net-feature-switch`. Network feature switch status.
    pub fn net_feature_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/net/net-feature-switch")
    }

    /// `net/cell-info`. Cell information.
    pub fn cell_info(&self) -> Result<Value> {
        get_value(self.conn, "api/net/cell-info")
    }

    /// `net/csps_state`. CSPS state.
    pub fn csps_state(&self) -> Result<Value> {
        get_value(self.conn, "api/net/csps_state")
    }

    /// Reconnect to the network.
    pub fn reconnect(&self) -> Result<String> {
        let body: XmlMap = map_of([("ReconnectAction", "1".to_string())]);
        post_set(self.conn, "api/net/reconnect", &body)
    }

    /// `net/antenna-configuration`. 5G antenna configuration.
    ///
    /// Supplementary endpoint from
    /// Brovi-Huawei-5G-CPE-Manager.
    pub fn get_antenna_configuration(&self) -> Result<Value> {
        get_value(self.conn, "api/net/antenna-configuration")
    }

    /// `net/lock-cell`. Lock the device to a specific 5G cell.
    ///
    /// * `lock` — `true` to lock, `false` to unlock.
    /// * `freq` — the cell frequency (ARFCN), `0` when unlocking.
    /// * `pci` — the physical cell identity, `0` when unlocking.
    ///
    /// Supplementary endpoint from
    /// Brovi-Huawei-5G-CPE-Manager.
    pub fn set_lock_cell(&self, lock: bool, freq: i64, pci: i64) -> Result<String> {
        let body: XmlMap = map_of([
            (
                "LockCell",
                if lock {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("Freq", freq.to_string()),
            ("PCI", pci.to_string()),
        ]);
        post_set(self.conn, "api/net/lock-cell", &body)
    }
}
