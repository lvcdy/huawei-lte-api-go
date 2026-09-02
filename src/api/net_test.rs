//! Tests for the [`Net`] API group.

use crate::api::net::Net;
use crate::enums::net::NetworkMode;
use crate::testsupport::conn_with;

/// GET endpoints resolve into JSON.
#[test]
fn current_plmn_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/net/current-plmn",
        "<response><Name>China Mobile</Name><ShortName>CMCC</ShortName><Numeric>46000</Numeric></response>",
    );

    let net = Net::new(&conn);
    let value = net.current_plmn().expect("plmn ok");

    assert_eq!(value["Name"], "China Mobile");
    assert_eq!(value["Numeric"], "46000");
}

/// `set_net_mode` serialises the enum's string value and hex band masks.
#[test]
fn set_net_mode_serialises_enum_and_hex() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/net/net-mode", "<response>OK</response>");

    let net = Net::new(&conn);
    net.set_net_mode(0x7fffffff, 0xffffffff, NetworkMode::Mode4G3GAuto)
        .expect("set ok");

    let body = tx.body_string_for("api/net/net-mode");
    assert!(
        body.contains("<NetworkMode>0302</NetworkMode>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<NetworkBand>ffffffff</NetworkBand>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<LTEBand>7fffffff</LTEBand>"),
        "body was: {body}"
    );
}

/// `set_register` POSTs mode/plmn/rat.
#[test]
fn set_register_posts_mode_plmn_rat() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/net/register", "<response>OK</response>");

    let net = Net::new(&conn);
    net.set_register("1", "46000", "7").expect("set ok");

    let body = tx.body_string_for("api/net/register");
    assert!(body.contains("<Mode>1</Mode>"), "body was: {body}");
    assert!(body.contains("<Plmn>46000</Plmn>"), "body was: {body}");
    assert!(body.contains("<Rat>7</Rat>"), "body was: {body}");
}

/// `reconnect` POSTs the reconnect action.
#[test]
fn reconnect_posts_action() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/net/reconnect", "<response>OK</response>");

    let net = Net::new(&conn);
    net.reconnect().expect("reconnect ok");

    let body = tx.body_string_for("api/net/reconnect");
    assert!(
        body.contains("<ReconnectAction>1</ReconnectAction>"),
        "body was: {body}"
    );
}
