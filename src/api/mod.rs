//! The API groups, each mirroring one of the Python library's `api/*.py`
//! classes.
//!
//! Every group is a thin wrapper around a [`Connection`] exposing one Rust
//! method per Python method. Get-style methods return a [`Value`] (the JSON
//! translation of the device's XML response), set-style methods return the
//! response string (normally `"OK"`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::xml::{map_to_json, value_to_json, XmlMap, XmlValue};

pub mod app;
pub mod bluetooth;
pub mod cradle;
pub mod cwmp;
pub mod d_dns;
pub mod developer;
pub mod developermode;
pub mod device;
pub mod dhcp;
pub mod diagnosis;
pub mod dial_up;
pub mod file_manager;
pub mod global;
pub mod host;
pub mod lan;
pub mod language;
pub mod led;
pub mod log;
pub mod m_log;
pub mod monitoring;
pub mod net;
pub mod ntwk;
pub mod online_update;
pub mod ota;
pub mod pb;
pub mod pin;
pub mod redirection;
pub mod s_ntp;
pub mod sd_card;
pub mod security;
pub mod sms;
pub mod staticroute;
pub mod statistic;
pub mod syslog;
pub mod system;
pub mod time;
pub mod time_rule;
pub mod usb_printer;
pub mod usb_storage;
pub mod ussd;
pub mod v_sim;
pub mod voice;
pub mod vpn;
pub mod web_server;
pub mod wlan;

#[cfg(test)]
mod app_test;
#[cfg(test)]
mod bluetooth_test;
#[cfg(test)]
mod cradle_test;
#[cfg(test)]
mod cwmp_test;
#[cfg(test)]
mod d_dns_test;
#[cfg(test)]
mod developer_test;
#[cfg(test)]
mod developermode_test;
#[cfg(test)]
mod device_test;
#[cfg(test)]
mod dhcp_test;
#[cfg(test)]
mod diagnosis_test;
#[cfg(test)]
mod dial_up_test;
#[cfg(test)]
mod global_test;
#[cfg(test)]
mod host_test;
#[cfg(test)]
mod lan_test;
#[cfg(test)]
mod language_test;
#[cfg(test)]
mod led_test;
#[cfg(test)]
mod log_test;
#[cfg(test)]
mod m_log_test;
#[cfg(test)]
mod monitoring_test;
#[cfg(test)]
mod net_test;
#[cfg(test)]
mod ntwk_test;
#[cfg(test)]
mod online_update_test;
#[cfg(test)]
mod ota_test;
#[cfg(test)]
mod pb_test;
#[cfg(test)]
mod pin_test;
#[cfg(test)]
mod redirection_test;
#[cfg(test)]
mod s_ntp_test;
#[cfg(test)]
mod sd_card_test;
#[cfg(test)]
mod security_test;
#[cfg(test)]
mod sms_test;
#[cfg(test)]
mod staticroute_test;
#[cfg(test)]
mod statistic_test;
#[cfg(test)]
mod syslog_test;
#[cfg(test)]
mod system_test;
#[cfg(test)]
mod time_rule_test;
#[cfg(test)]
mod time_test;
#[cfg(test)]
mod usb_printer_test;
#[cfg(test)]
mod usb_storage_test;
#[cfg(test)]
mod ussd_test;
#[cfg(test)]
mod v_sim_test;
#[cfg(test)]
mod voice_test;
#[cfg(test)]
mod vpn_test;
#[cfg(test)]
mod web_server_test;
#[cfg(test)]
mod wlan_test;

/// Extract the `response` node of a parsed response as a JSON value.
///
/// Mirrors the Python `get()` behaviour of returning `data["response"]`
/// (falling back to the whole map when there is no `response` key).
pub(crate) fn resp_value(map: &XmlMap) -> Value {
    match map.get("response") {
        Some(XmlValue::Map(m)) => map_to_json(m),
        Some(v) => value_to_json(v),
        None => map_to_json(map),
    }
}

/// Run a GET endpoint and return its decoded JSON value.
pub(crate) fn get_value(conn: &Connection, endpoint: &str) -> Result<Value> {
    let data = conn.session().get_xml(endpoint)?;
    Ok(resp_value(&data))
}

/// Extract the response text (e.g. `"OK"`) from a set-response.
pub(crate) fn resp_string(map: &XmlMap) -> String {
    map.get("response")
        .map(XmlValue::as_str)
        .unwrap_or_default()
}

/// Run a POST endpoint with an XML body and return the response text.
pub(crate) fn post_set(conn: &Connection, endpoint: &str, body: &XmlMap) -> Result<String> {
    let resp = conn.session().post_xml(endpoint, body)?;
    Ok(resp_string(&resp))
}

/// Like [`post_set`] but forces a CSRF refresh after the request.
pub(crate) fn post_set_refresh(conn: &Connection, endpoint: &str, body: &XmlMap) -> Result<String> {
    let resp = conn.session().post_xml_refresh(endpoint, body)?;
    Ok(resp_string(&resp))
}

/// Run a POST endpoint with an XML body and return the decoded JSON value
/// (Python `post_get` — used when a POST returns a data payload).
pub(crate) fn post_get_value(conn: &Connection, endpoint: &str, body: &XmlMap) -> Result<Value> {
    let resp = conn.session().post_xml(endpoint, body)?;
    Ok(resp_value(&resp))
}
