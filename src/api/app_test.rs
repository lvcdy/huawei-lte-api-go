//! Tests for the [`App`] API group.

use crate::api::app::App;
use crate::testsupport::conn_with;

/// `app/operatorinfo` resolves.
#[test]
fn operatorinfo_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/app/operatorinfo",
        "<response><OperatorName>CMCC</OperatorName></response>",
    );

    let app = App::new(&conn);
    let value = app.operatorinfo("en").expect("ok");
    assert_eq!(value["OperatorName"], "CMCC");
}

/// `app/privacypolicy` resolves.
#[test]
fn privacypolicy_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/app/privacypolicy",
        "<response><Approve>0</Approve></response>",
    );

    let app = App::new(&conn);
    let value = app.privacypolicy("en").expect("ok");
    assert_eq!(value["Approve"], "0");
}

/// `accept_privacypolicy` posts a nested `data` map with Approve=2.
#[test]
fn accept_privacypolicy_posts_approve() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/app/privacypolicy",
        "<response><Approve>2</Approve></response>",
    );

    let app = App::new(&conn);
    let value = app.accept_privacypolicy(true).expect("ok");
    assert_eq!(value["Approve"], "2");

    let body = tx.body_string_for("api/app/privacypolicy");
    assert!(body.contains("<data>"), "body was: {body}");
    assert!(body.contains("<Approve>2</Approve>"), "body was: {body}");
    assert!(body.contains("<Liscence>0</Liscence>"), "body was: {body}");
}
