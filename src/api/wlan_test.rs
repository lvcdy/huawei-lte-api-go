//! Tests for the [`WLan`] API group.

use crate::api::wlan::WLan;
use crate::testsupport::conn_with;
use crate::tools::map_of;
use crate::xml::XmlMap;

/// GET endpoints resolve into JSON.
#[test]
fn basic_settings_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/wlan/basic-settings",
        "<response><WifiSsid>MyRouter</WifiSsid><WifiHide>0</WifiHide></response>",
    );

    let wlan = WLan::new(&conn);
    let value = wlan.basic_settings().expect("settings ok");

    assert_eq!(value["WifiSsid"], "MyRouter");
    assert_eq!(value["WifiHide"], "0");
}

/// `set_basic_settings` serialises booleans as 1/0.
#[test]
fn set_basic_settings_serialises_booleans() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/wlan/basic-settings", "<response>OK</response>");

    let wlan = WLan::new(&conn);
    wlan.set_basic_settings("MyRouter", true, false)
        .expect("set ok");

    let body = tx.body_string_for("api/wlan/basic-settings");
    assert!(
        body.contains("<WifiSsid>MyRouter</WifiSsid>"),
        "body was: {body}"
    );
    assert!(body.contains("<WifiHide>1</WifiHide>"), "body was: {body}");
    assert!(
        body.contains("<WifiRestart>0</WifiRestart>"),
        "body was: {body}"
    );
}

/// `set_security_settings` serialises all fields.
#[test]
fn set_security_settings_serialises_fields() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/wlan/security-settings", "<response>OK</response>");

    let wlan = WLan::new(&conn);
    wlan.set_security_settings("secret123", "", 1, 0, 3, true)
        .expect("set ok");

    let body = tx.body_string_for("api/wlan/security-settings");
    assert!(
        body.contains("<WifiAuthmode>3</WifiAuthmode>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<WifiWpapsk>secret123</WifiWpapsk>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<WifiWpaencryptionmodes>1</WifiWpaencryptionmodes>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<WifiRestart>1</WifiRestart>"),
        "body was: {body}"
    );
}

/// `set_multi_basic_settings` wraps clients into a `Ssids` list.
#[test]
fn set_multi_basic_settings_wraps_ssids() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/wlan/multi-basic-settings", "<response>OK</response>");

    let client: XmlMap = map_of([
        ("wifihostname", "Guest".to_string()),
        ("WifiMacFilterMac", "AA:BB:CC:DD:EE:FF".to_string()),
    ]);

    let wlan = WLan::new(&conn);
    wlan.set_multi_basic_settings(&[client]).expect("set ok");

    let body = tx.body_string_for("api/wlan/multi-basic-settings");
    assert!(body.contains("<Ssids>"), "body was: {body}");
    assert!(
        body.contains("<wifihostname>Guest</wifihostname>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<WifiMacFilterMac>AA:BB:CC:DD:EE:FF</WifiMacFilterMac>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<WifiRestart>1</WifiRestart>"),
        "body was: {body}"
    );
}
