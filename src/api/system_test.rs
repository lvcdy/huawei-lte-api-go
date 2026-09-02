//! Tests for the [`System`] API group.

use crate::api::system::System;
use crate::testsupport::conn_with;

/// GET endpoints resolve into JSON.
#[test]
fn deviceinfo_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/system/deviceinfo",
        "<response><DeviceName>B818</DeviceName><SoftwareVersion>21.180.05.00.00</SoftwareVersion></response>",
    );

    let sys = System::new(&conn);
    let value = sys.deviceinfo().expect("deviceinfo ok");

    assert_eq!(value["DeviceName"], "B818");
    assert_eq!(value["SoftwareVersion"], "21.180.05.00.00");
}

/// `onlineupg` POSTs the nested `action`/`data` body.
#[test]
fn onlineupg_posts_nested_body() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/system/onlineupg",
        "<response><State>0</State></response>",
    );

    let sys = System::new(&conn);
    let value = sys.onlineupg().expect("onlineupg ok");
    assert_eq!(value["State"], "0");

    let body = tx.body_string_for("api/system/onlineupg");
    assert!(body.contains("<action>check</action>"), "body was: {body}");
    assert!(
        body.contains("<UpdateAction>1</UpdateAction>"),
        "body was: {body}"
    );
}

/// `devcapacity` resolves nested capacity payload.
#[test]
fn devcapacity_nested() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/system/devcapacity",
        "<response><Memory><Total>512</Total></Memory></response>",
    );

    let sys = System::new(&conn);
    let value = sys.devcapacity().expect("devcapacity ok");
    assert_eq!(value["Memory"]["Total"], "512");
}
