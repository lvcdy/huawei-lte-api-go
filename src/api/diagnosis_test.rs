//! Tests for the [`Diagnosis`] API group.

use crate::api::diagnosis::Diagnosis;
use crate::testsupport::conn_with;

/// GET endpoints resolve into JSON.
#[test]
fn trace_route_result_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/diagnosis/tracerouteresult",
        "<response><HopCount>3</HopCount></response>",
    );

    let diag = Diagnosis::new(&conn);
    let value = diag.trace_route_result().expect("trace ok");
    assert_eq!(value["HopCount"], "3");
}

/// `set_diagnose_ping` POSTs host + timeout.
#[test]
fn set_diagnose_ping_posts_host_timeout() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/diagnosis/diagnose_ping", "<response>OK</response>");

    let diag = Diagnosis::new(&conn);
    diag.set_diagnose_ping("8.8.8.8", 4000).expect("ping ok");

    let body = tx.body_string_for("api/diagnosis/diagnose_ping");
    assert!(body.contains("<Host>8.8.8.8</Host>"), "body was: {body}");
    assert!(body.contains("<Timeout>4000</Timeout>"), "body was: {body}");
}

/// `set_diagnose_traceroute` POSTs host, timeout and hop count.
#[test]
fn set_diagnose_traceroute_posts_all() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/diagnosis/diagnose_traceroute",
        "<response>OK</response>",
    );

    let diag = Diagnosis::new(&conn);
    diag.set_diagnose_traceroute("example.com", 4000, 30)
        .expect("traceroute ok");

    let body = tx.body_string_for("api/diagnosis/diagnose_traceroute");
    assert!(
        body.contains("<Host>example.com</Host>"),
        "body was: {body}"
    );
    assert!(body.contains("<Timeout>4000</Timeout>"), "body was: {body}");
    assert!(
        body.contains("<MaxHopCount>30</MaxHopCount>"),
        "body was: {body}"
    );
}
