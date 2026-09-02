//! Tests for the [`Dhcp`] API group.

use crate::api::dhcp::Dhcp;
use crate::testsupport::conn_with;

/// GET endpoints resolve into JSON.
#[test]
fn settings_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/dhcp/settings",
        "<response><DhcpStatus>1</DhcpStatus><DhcpIPAddress>192.168.8.1</DhcpIPAddress></response>",
    );

    let dhcp = Dhcp::new(&conn);
    let value = dhcp.settings().expect("settings ok");
    assert_eq!(value["DhcpStatus"], "1");
    assert_eq!(value["DhcpIPAddress"], "192.168.8.1");
}

/// `set_settings` derives the start/end IP range from the base address.
#[test]
fn set_settings_derives_ip_range() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/dhcp/settings", "<response>OK</response>");

    let dhcp = Dhcp::new(&conn);
    dhcp.set_settings(
        "192.168.8.1",
        "255.255.255.0",
        true,
        100,
        200,
        86400,
        true,
        Some("8.8.8.8"),
        Some("8.8.4.4"),
        true,
    )
    .expect("set ok");

    let body = tx.body_string_for("api/dhcp/settings");
    assert!(
        body.contains("<DhcpIPAddress>192.168.8.1</DhcpIPAddress>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<DhcpStatus>1</DhcpStatus>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<DhcpStartIPAddress>192.168.8.100</DhcpStartIPAddress>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<DhcpEndIPAddress>192.168.8.200</DhcpEndIPAddress>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<PrimaryDns>8.8.8.8</PrimaryDns>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<SecondaryDns>8.8.4.4</SecondaryDns>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<DhcpLeaseTime>86400</DhcpLeaseTime>"),
        "body was: {body}"
    );
}

/// `set_settings` uses empty DNS strings when not provided.
#[test]
fn set_settings_empty_dns_when_absent() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/dhcp/settings", "<response>OK</response>");

    let dhcp = Dhcp::new(&conn);
    dhcp.set_settings(
        "10.0.0.1",
        "255.255.255.0",
        false,
        2,
        254,
        3600,
        false,
        None,
        None,
        false,
    )
    .expect("set ok");

    let body = tx.body_string_for("api/dhcp/settings");
    assert!(
        body.contains("<DhcpStatus>0</DhcpStatus>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<DhcpStartIPAddress>10.0.0.2</DhcpStartIPAddress>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<PrimaryDns></PrimaryDns>"),
        "body was: {body}"
    );
}
