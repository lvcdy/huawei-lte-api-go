//! Tests for the [`Vpn`] API group.

use crate::api::vpn::Vpn;
use crate::testsupport::conn_with;

/// `vpn/feature-switch` resolves.
#[test]
fn feature_switch_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/vpn/feature-switch",
        "<response><FeatureSwitch>1</FeatureSwitch></response>",
    );

    let vpn = Vpn::new(&conn);
    let value = vpn.feature_switch().expect("ok");
    assert_eq!(value["FeatureSwitch"], "1");
}

/// `vpn/br_list` resolves.
#[test]
fn br_list_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/vpn/br_list",
        "<response><BrList>br0</BrList></response>",
    );

    let vpn = Vpn::new(&conn);
    let value = vpn.br_list().expect("ok");
    assert_eq!(value["BrList"], "br0");
}

/// `vpn/ipsec_settings` resolves.
#[test]
fn ipsec_settings_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/vpn/ipsec_settings",
        "<response><IpsecSettings>1</IpsecSettings></response>",
    );

    let vpn = Vpn::new(&conn);
    let value = vpn.ipsec_settings().expect("ok");
    assert_eq!(value["IpsecSettings"], "1");
}

/// `vpn/l2tp_settings` resolves.
#[test]
fn l2tp_settings_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/vpn/l2tp_settings",
        "<response><L2tpSettings>0</L2tpSettings></response>",
    );

    let vpn = Vpn::new(&conn);
    let value = vpn.l2tp_settings().expect("ok");
    assert_eq!(value["L2tpSettings"], "0");
}

/// `vpn/pptp_settings` resolves.
#[test]
fn pptp_settings_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/vpn/pptp_settings",
        "<response><PptpSettings>1</PptpSettings></response>",
    );

    let vpn = Vpn::new(&conn);
    let value = vpn.pptp_settings().expect("ok");
    assert_eq!(value["PptpSettings"], "1");
}

/// `toggle_status` posts enable flag to the selected vpn settings endpoint.
#[test]
fn toggle_status_posts_enable() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/vpn/l2tp_settings", "<response>OK</response>");

    let vpn = Vpn::new(&conn);
    vpn.toggle_status(true, "l2tp").expect("ok");

    let body = tx.body_string_for("api/vpn/l2tp_settings");
    assert!(body.contains("<enable>1</enable>"), "body was: {body}");
}

/// `vpn/status` resolves.
#[test]
fn status_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/vpn/status",
        "<response><VpnStatus>1</VpnStatus></response>",
    );

    let vpn = Vpn::new(&conn);
    let value = vpn.status().expect("ok");
    assert_eq!(value["VpnStatus"], "1");
}
