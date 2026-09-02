//! Tests for the [`Cradle`] API group.

use crate::api::cradle::Cradle;
use crate::testsupport::conn_with;

/// `cradle/status-info` resolves.
#[test]
fn status_info_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/cradle/status-info",
        "<response><CradleStatus>1</CradleStatus></response>",
    );

    let cradle = Cradle::new(&conn);
    let value = cradle.status_info().expect("status ok");
    assert_eq!(value["CradleStatus"], "1");
}

/// `cradle/feature-switch` resolves.
#[test]
fn feature_switch_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/cradle/feature-switch",
        "<response><FeatureSwitch>0</FeatureSwitch></response>",
    );

    let cradle = Cradle::new(&conn);
    let value = cradle.feature_switch().expect("feature switch ok");
    assert_eq!(value["FeatureSwitch"], "0");
}

/// `cradle/basic-info` resolves.
#[test]
fn basic_info_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/cradle/basic-info",
        "<response><DeviceName>cradle</DeviceName></response>",
    );

    let cradle = Cradle::new(&conn);
    let value = cradle.basic_info().expect("basic info ok");
    assert_eq!(value["DeviceName"], "cradle");
}

/// `cradle/factory-mac` resolves.
#[test]
fn factory_mac_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/cradle/factory-mac",
        "<response><MacAddress>00:11:22:33:44:55</MacAddress></response>",
    );

    let cradle = Cradle::new(&conn);
    let value = cradle.factory_mac().expect("factory mac ok");
    assert_eq!(value["MacAddress"], "00:11:22:33:44:55");
}

/// `cradle/mac-info` resolves.
#[test]
fn mac_info_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/cradle/mac-info",
        "<response><MacInfo>info</MacInfo></response>",
    );

    let cradle = Cradle::new(&conn);
    let value = cradle.mac_info().expect("mac info ok");
    assert_eq!(value["MacInfo"], "info");
}
