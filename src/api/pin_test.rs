//! Tests for the [`Pin`] API group.

use crate::api::pin::Pin;
use crate::testsupport::conn_with;

/// `pin/status` resolves.
#[test]
fn status_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/pin/status",
        "<response><PinStatus>0</PinStatus><SimState>1</SimState></response>",
    );

    let pin = Pin::new(&conn);
    let value = pin.status().expect("status ok");
    assert_eq!(value["PinStatus"], "0");
    assert_eq!(value["SimState"], "1");
}

/// `operate` POSTs the operate type and pin fields.
#[test]
fn operate_posts_pin_fields() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/pin/operate", "<response>OK</response>");

    let pin = Pin::new(&conn);
    pin.operate("1", Some("1234"), Some("5678"), None)
        .expect("operate ok");

    let body = tx.body_string_for("api/pin/operate");
    assert!(
        body.contains("<OperateType>1</OperateType>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<CurrentPin>1234</CurrentPin>"),
        "body was: {body}"
    );
    assert!(body.contains("<NewPin>5678</NewPin>"), "body was: {body}");
    assert!(body.contains("<PukCode></PukCode>"), "body was: {body}");
}

/// `operate` uses empty strings for absent optional fields.
#[test]
fn operate_uses_empty_when_absent() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/pin/operate", "<response>OK</response>");

    let pin = Pin::new(&conn);
    pin.operate("4", None, None, Some("12345678"))
        .expect("operate ok");

    let body = tx.body_string_for("api/pin/operate");
    assert!(
        body.contains("<CurrentPin></CurrentPin>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<PukCode>12345678</PukCode>"),
        "body was: {body}"
    );
}
