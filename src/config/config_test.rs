//! Tests for the [`config`] API groups.
//!
//! Every config group is a set of GET-only endpoints under the `config/`
//! prefix, so these tests exercise routing, CSRF tokens and JSON decoding
//! for a representative sample of groups.

use crate::config::device::DeviceConfig;
use crate::config::global::GlobalConfig;
use crate::config::lan::LanConfig;
use crate::config::network::NetworkConfig;
use crate::config::sms::SmsConfig;
use crate::config::wifi::WifiConfig;
use crate::testsupport::conn_with;

/// `config/device/config.xml` resolves the payload.
#[test]
fn device_config_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "config/device/config.xml",
        "<response><DeviceName>B818</DeviceName></response>",
    );

    let cfg = DeviceConfig::new(&conn);
    let value = cfg.config().expect("device config ok");
    assert_eq!(value["DeviceName"], "B818");
}

/// `config/wifi/config.xml` resolves a nested payload.
#[test]
fn wifi_config_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "config/wifi/config.xml",
        "<response><Wifi><WifiSsid>MyRouter</WifiSsid></Wifi></response>",
    );

    let cfg = WifiConfig::new(&conn);
    let value = cfg.config().expect("wifi config ok");
    assert_eq!(value["Wifi"]["WifiSsid"], "MyRouter");
}

/// A `config/` GET carries the CSRF token discovered at session init.
#[test]
fn config_get_carries_csrf_token() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "config/lan/config.xml",
        "<response><LanIpAddress>192.168.8.1</LanIpAddress></response>",
    );

    let cfg = LanConfig::new(&conn);
    let value = cfg.config().expect("lan config ok");
    assert_eq!(value["LanIpAddress"], "192.168.8.1");

    let req = tx
        .requests()
        .into_iter()
        .find(|r| r.url.contains("config/lan/config.xml"))
        .expect("lan config request recorded");
    assert_eq!(req.token.as_deref(), Some("CSRF_TOKEN_HOME"));
}

/// `config/network/net-mode.xml` resolves.
#[test]
fn network_config_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "config/network/net-mode.xml",
        "<response><NetworkMode>0302</NetworkMode></response>",
    );

    let cfg = NetworkConfig::new(&conn);
    let value = cfg.net_mode().expect("net-mode ok");
    assert_eq!(value["NetworkMode"], "0302");
}

/// `config/global/config.xml` resolves a nested payload.
#[test]
fn global_config_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "config/global/config.xml",
        "<response><Language>zh-cn</Language></response>",
    );

    let cfg = GlobalConfig::new(&conn);
    let value = cfg.config().expect("global config ok");
    assert_eq!(value["Language"], "zh-cn");
}

/// `config/sms/config.xml` resolves.
#[test]
fn sms_config_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "config/sms/config.xml",
        "<response><LocalSave>1</LocalSave></response>",
    );

    let cfg = SmsConfig::new(&conn);
    let value = cfg.config().expect("sms config ok");
    assert_eq!(value["LocalSave"], "1");
}
