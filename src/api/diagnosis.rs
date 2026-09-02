//! Diagnosis API group (`api/Diagnosis.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_set};

/// Diagnosis API group.
pub struct Diagnosis<'a> {
    conn: &'a Connection,
}

impl<'a> Diagnosis<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Diagnosis { conn }
    }

    /// `diagnosis/tracerouteresult`. Traceroute diagnosis result.
    pub fn trace_route_result(&self) -> Result<Value> {
        get_value(self.conn, "api/diagnosis/tracerouteresult")
    }

    /// `diagnosis/diagnose_ping`. Ping diagnosis result.
    pub fn diagnose_ping(&self) -> Result<Value> {
        get_value(self.conn, "api/diagnosis/diagnose_ping")
    }

    /// Start a ping diagnosis.
    ///
    /// * `host` — host to ping.
    /// * `timeout` — timeout in milliseconds (default 4000).
    pub fn set_diagnose_ping(&self, host: &str, timeout: i64) -> Result<String> {
        let body: XmlMap = map_of([("Host", host.to_string()), ("Timeout", timeout.to_string())]);
        post_set(self.conn, "api/diagnosis/diagnose_ping", &body)
    }

    /// `diagnosis/diagnose_traceroute`. Traceroute diagnosis result.
    pub fn diagnose_traceroute(&self) -> Result<Value> {
        get_value(self.conn, "api/diagnosis/diagnose_traceroute")
    }

    /// Start a traceroute diagnosis.
    ///
    /// * `host` — host to traceroute.
    /// * `timeout` — timeout in milliseconds (default 4000).
    /// * `max_hop_count` — maximum hop count (default 30).
    pub fn set_diagnose_traceroute(
        &self,
        host: &str,
        timeout: i64,
        max_hop_count: i64,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            ("Host", host.to_string()),
            ("MaxHopCount", max_hop_count.to_string()),
            ("Timeout", timeout.to_string()),
        ]);
        post_set(self.conn, "api/diagnosis/diagnose_traceroute", &body)
    }

    /// `diagnosis/time_reboot`. Time reboot status.
    pub fn time_reboot(&self) -> Result<Value> {
        get_value(self.conn, "api/diagnosis/time_reboot")
    }

    /// `diagnosis/get-wan-service-name`. WAN service name.
    pub fn wan_service_name(&self) -> Result<Value> {
        get_value(self.conn, "api/diagnosis/get-wan-service-name")
    }
}
