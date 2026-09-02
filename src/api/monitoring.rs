//! Monitoring API group (`api/Monitoring.py`).
//!
//! Device status, traffic statistics and usage-accounting settings.

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_set};

/// Monitoring API group.
pub struct Monitoring<'a> {
    conn: &'a Connection,
}

impl<'a> Monitoring<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Monitoring { conn }
    }

    /// `monitoring/converged-status`. Converged status of the router.
    pub fn converged_status(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/converged-status")
    }

    /// `monitoring/status`. Status info (signal in `SignalIcon`/`SignalIconNr`).
    pub fn status(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/status")
    }

    /// `monitoring/check-notifications`. Pending notifications.
    pub fn check_notifications(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/check-notifications")
    }

    /// `monitoring/traffic-statistics`. Traffic statistics.
    pub fn traffic_statistics(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/traffic-statistics")
    }

    /// `monitoring/start_date`. Current monitoring start date.
    pub fn start_date(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/start_date")
    }

    /// Set the LTE usage alarm.
    ///
    /// * `start_day` — day of month the accounting starts.
    /// * `data_limit` — e.g. `"1GB"`.
    /// * `month_threshold` — alarm percent (e.g. `90`).
    pub fn set_start_date(
        &self,
        start_day: i64,
        data_limit: &str,
        month_threshold: i64,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            ("StartDay", start_day.to_string()),
            ("DataLimit", data_limit.to_string()),
            ("MonthThreshold", month_threshold.to_string()),
            ("SetMonthData", "1".to_string()),
        ]);
        post_set(self.conn, "api/monitoring/start_date", &body)
    }

    /// `monitoring/start_date_wlan`. WLAN monitoring start date.
    pub fn start_date_wlan(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/start_date_wlan")
    }

    /// Set the WLAN usage alarm.
    pub fn set_start_date_wlan(
        &self,
        start_day: i64,
        data_limit: &str,
        month_threshold: i64,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            ("StartDay", start_day.to_string()),
            ("DataLimit", data_limit.to_string()),
            ("MonthThreshold", month_threshold.to_string()),
            ("SettingEnable", "1".to_string()),
        ]);
        post_set(self.conn, "api/monitoring/start_date_wlan", &body)
    }

    /// `monitoring/month_statistics`. Monthly statistics.
    pub fn month_statistics(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/month_statistics")
    }

    /// `monitoring/month_statistics_wlan`. Monthly WLAN statistics.
    pub fn month_statistics_wlan(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/month_statistics_wlan")
    }

    /// Clear traffic statistics.
    pub fn set_clear_traffic(&self) -> Result<String> {
        let body: XmlMap = map_of([("ClearTraffic", "1".to_string())]);
        post_set(self.conn, "api/monitoring/clear-traffic", &body)
    }

    /// `monitoring/wifi-month-setting` (reverse engineered, likely unused).
    pub fn wifi_month_setting(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/wifi-month-setting")
    }

    /// `monitoring/daily-data-limit`. Daily data limit.
    pub fn daily_data_limit(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/daily-data-limit")
    }

    /// `monitoring/statistic-feature-switch`. Statistic feature switch state.
    pub fn statistic_feature_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/statistic-feature-switch")
    }

    /// `monitoring/onekey_diag`. One-key diagnostic state.
    pub fn onekey_diag(&self) -> Result<Value> {
        get_value(self.conn, "api/monitoring/onekey_diag")
    }
}
