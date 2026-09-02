//! Tests for the [`DialUp`] API group.

use crate::api::dial_up::DialUp;
use crate::testsupport::conn_with;

/// `dialup/mobile-dataswitch` resolves.
#[test]
fn mobile_dataswitch_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/dialup/mobile-dataswitch",
        "<response><dataswitch>1</dataswitch></response>",
    );

    let du = DialUp::new(&conn);
    let value = du.mobile_dataswitch().expect("ok");
    assert_eq!(value["dataswitch"], "1");
}

/// `dialup/connection` resolves.
#[test]
fn connection_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/dialup/connection",
        "<response><ConnectionStatus>1</ConnectionStatus></response>",
    );

    let du = DialUp::new(&conn);
    let value = du.connection().expect("ok");
    assert_eq!(value["ConnectionStatus"], "1");
}

/// `dialup/dialup-feature-switch` resolves.
#[test]
fn dialup_feature_switch_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/dialup/dialup-feature-switch",
        "<response><FeatureSwitch>1</FeatureSwitch></response>",
    );

    let du = DialUp::new(&conn);
    let value = du.dialup_feature_switch().expect("ok");
    assert_eq!(value["FeatureSwitch"], "1");
}

/// `dialup/profiles` resolves.
#[test]
fn profiles_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/dialup/profiles",
        "<response><ProfileCount>2</ProfileCount></response>",
    );

    let du = DialUp::new(&conn);
    let value = du.profiles().expect("ok");
    assert_eq!(value["ProfileCount"], "2");
}

/// `dialup/auto-apn` resolves.
#[test]
fn auto_apn_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/dialup/auto-apn",
        "<response><AutoApn>1</AutoApn></response>",
    );

    let du = DialUp::new(&conn);
    let value = du.auto_apn().expect("ok");
    assert_eq!(value["AutoApn"], "1");
}

/// `dial` posts `Action=1`.
#[test]
fn dial_posts_action() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/dialup/dial", "<response>OK</response>");

    let du = DialUp::new(&conn);
    du.dial().expect("ok");

    let body = tx.body_string_for("api/dialup/dial");
    assert!(body.contains("<Action>1</Action>"), "body was: {body}");
}

/// `set_mobile_dataswitch` posts the switch value.
#[test]
fn set_mobile_dataswitch_posts_value() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/dialup/mobile-dataswitch", "<response>OK</response>");

    let du = DialUp::new(&conn);
    du.set_mobile_dataswitch(0).expect("ok");

    let body = tx.body_string_for("api/dialup/mobile-dataswitch");
    assert!(
        body.contains("<dataswitch>0</dataswitch>"),
        "body was: {body}"
    );
}

/// `set_default_profile` posts SetDefault/Delete/Modify.
#[test]
fn set_default_profile_posts_flags() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/dialup/profiles", "<response>OK</response>");

    let du = DialUp::new(&conn);
    du.set_default_profile(3).expect("ok");

    let body = tx.body_string_for("api/dialup/profiles");
    assert!(
        body.contains("<SetDefault>3</SetDefault>"),
        "body was: {body}"
    );
    assert!(body.contains("<Delete>0</Delete>"), "body was: {body}");
    assert!(body.contains("<Modify>0</Modify>"), "body was: {body}");
}

/// `delete_profile` posts the delete index.
#[test]
fn delete_profile_posts_index() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/dialup/profiles", "<response>OK</response>");

    let du = DialUp::new(&conn);
    du.delete_profile(5).expect("ok");

    let body = tx.body_string_for("api/dialup/profiles");
    assert!(
        body.contains("<SetDefault>0</SetDefault>"),
        "body was: {body}"
    );
    assert!(body.contains("<Delete>5</Delete>"), "body was: {body}");
}

/// `create_profile` posts the full profile map.
#[test]
fn create_profile_posts_profile() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/dialup/profiles", "<response>OK</response>");

    let du = DialUp::new(&conn);
    du.create_profile(
        "MyAPN",
        Some("user"),
        Some("pass"),
        Some("internet"),
        None,
        "PAP",
        "IPV4",
        true,
    )
    .expect("ok");

    let body = tx.body_string_for("api/dialup/profiles");
    assert!(
        body.contains("<SetDefault>1</SetDefault>"),
        "body was: {body}"
    );
    assert!(body.contains("<Modify>1</Modify>"), "body was: {body}");
    assert!(body.contains("<Name>MyAPN</Name>"), "body was: {body}");
    assert!(
        body.contains("<ApnName>internet</ApnName>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<ApnIsStatic>1</ApnIsStatic>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<Username>user</Username>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<Password>pass</Password>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<AuthMode>PAP</AuthMode>"),
        "body was: {body}"
    );
    assert!(body.contains("<iptype>IPV4</iptype>"), "body was: {body}");
}
