//! Tests for the [`Developer`] API group.

use crate::api::developer::Developer;
use crate::testsupport::conn_with;

/// GET `developer/developermode-featureswitch` resolves.
#[test]
fn developermode_featureswitch_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/developer/developermode-featureswitch",
        "<response><FeatureSwitch>1</FeatureSwitch></response>",
    );

    let dev = Developer::new(&conn);
    let value = dev.developermode_featureswitch().expect("ok");
    assert_eq!(value["FeatureSwitch"], "1");
}

/// GET `app/atport-status` resolves.
#[test]
fn atport_status_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/app/atport-status",
        "<response><AtPortStatus>0</AtPortStatus></response>",
    );

    let dev = Developer::new(&conn);
    let value = dev.atport_status().expect("ok");
    assert_eq!(value["AtPortStatus"], "0");
}

/// POST `developer/atport-status` with `enable=true` sends `<enable>1</enable>`.
#[test]
fn set_atport_status_enable() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/developer/atport-status", "<response>OK</response>");

    let dev = Developer::new(&conn);
    dev.set_atport_status(true).expect("ok");

    let body = tx.body_string_for("api/developer/atport-status");
    assert!(body.contains("<enable>1</enable>"), "body was: {body}");
}

/// POST `developer/atport-status` with `enable=false` sends `<enable>0</enable>`.
#[test]
fn set_atport_status_disable() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/developer/atport-status", "<response>OK</response>");

    let dev = Developer::new(&conn);
    dev.set_atport_status(false).expect("ok");

    let body = tx.body_string_for("api/developer/atport-status");
    assert!(body.contains("<enable>0</enable>"), "body was: {body}");
}
