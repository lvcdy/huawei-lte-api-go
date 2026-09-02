//! Tests for the [`Security`] API group.

use crate::api::security::Security;
use crate::testsupport::conn_with;
use crate::tools::map_of;

/// GET endpoints resolve into JSON.
#[test]
fn firewall_switch_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/security/firewall-switch",
        "<response><FirewallMainSwitch>1</FirewallMainSwitch></response>",
    );

    let sec = Security::new(&conn);
    let value = sec.get_firewall_switch().expect("fw ok");
    assert_eq!(value["FirewallMainSwitch"], "1");
}

/// `set_firewall_switch` serialises each boolean as 1/0.
#[test]
fn set_firewall_switch_serialises_booleans() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/security/firewall-switch", "<response>OK</response>");

    let sec = Security::new(&conn);
    sec.set_firewall_switch(true, false, true, false, true)
        .expect("set ok");

    let body = tx.body_string_for("api/security/firewall-switch");
    assert!(
        body.contains("<FirewallMainSwitch>1</FirewallMainSwitch>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<FirewallIPFilterSwitch>0</FirewallIPFilterSwitch>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<FirewallWanPortPingSwitch>1</FirewallWanPortPingSwitch>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<firewallurlfilterswitch>0</firewallurlfilterswitch>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<firewallmacfilterswitch>1</firewallmacfilterswitch>"),
        "body was: {body}"
    );
}

/// `set_dmz` POSTs status + IP address.
#[test]
fn set_dmz_posts_status_and_ip() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/security/dmz", "<response>OK</response>");

    let sec = Security::new(&conn);
    sec.set_dmz(true, "192.168.8.100").expect("set ok");

    let body = tx.body_string_for("api/security/dmz");
    assert!(
        body.contains("<DmzStatus>1</DmzStatus>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<DmzIPAddress>192.168.8.100</DmzIPAddress>"),
        "body was: {body}"
    );
}

/// `set_sip` POSTs status + port.
#[test]
fn set_sip_posts_status_and_port() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/security/sip", "<response>OK</response>");

    let sec = Security::new(&conn);
    sec.set_sip(false, 5060).expect("set ok");

    let body = tx.body_string_for("api/security/sip");
    assert!(
        body.contains("<SipStatus>0</SipStatus>"),
        "body was: {body}"
    );
    assert!(body.contains("<SipPort>5060</SipPort>"), "body was: {body}");
}

/// `set_url_filter` forwards the given map unchanged.
#[test]
fn set_url_filter_forwards_map() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/security/url-filter", "<response>OK</response>");

    let filter: crate::xml::XmlMap = map_of([("Url", "example.com".to_string())]);

    let sec = Security::new(&conn);
    sec.set_url_filter(&filter).expect("set ok");

    let body = tx.body_string_for("api/security/url-filter");
    assert!(body.contains("<Url>example.com</Url>"), "body was: {body}");
}
