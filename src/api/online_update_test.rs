//! Tests for the [`OnlineUpdate`] API group.

use crate::api::online_update::OnlineUpdate;
use crate::testsupport::conn_with;

/// GET `online-update/check-new-version` resolves.
#[test]
fn check_new_version_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/online-update/check-new-version",
        "<response><State>0</State></response>",
    );

    let ou = OnlineUpdate::new(&conn);
    let value = ou.check_new_version().expect("ok");
    assert_eq!(value["State"], "0");
}

/// POST `online-update/check-new-version` with empty body.
#[test]
fn set_check_new_version_posts() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/online-update/check-new-version",
        "<response>OK</response>",
    );

    let ou = OnlineUpdate::new(&conn);
    ou.set_check_new_version().expect("ok");

    let body = tx.body_string_for("api/online-update/check-new-version");
    assert!(body.contains("<request>"), "body was: {body}");
    assert!(
        !body.contains("<check"),
        "expected no extra fields, body was: {body}"
    );
}

/// `online-update/status` resolves.
#[test]
fn status_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/online-update/status",
        "<response><UpgradeStatus>0</UpgradeStatus></response>",
    );

    let ou = OnlineUpdate::new(&conn);
    let value = ou.status().expect("ok");
    assert_eq!(value["UpgradeStatus"], "0");
}

/// `online-update/url-list` resolves.
#[test]
fn url_list_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/online-update/url-list",
        "<response><Url>http://upd.example</Url></response>",
    );

    let ou = OnlineUpdate::new(&conn);
    let value = ou.url_list().expect("ok");
    assert_eq!(value["Url"], "http://upd.example");
}

/// POST `online-update/ack-newversion` sends the user acknowledgement flag.
#[test]
fn set_ack_newversion_posts_flag() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/online-update/ack-newversion",
        "<response>OK</response>",
    );

    let ou = OnlineUpdate::new(&conn);
    ou.set_ack_newversion().expect("ok");

    let body = tx.body_string_for("api/online-update/ack-newversion");
    assert!(
        body.contains("<userAckNewVersion>0</userAckNewVersion>"),
        "body was: {body}"
    );
}

/// POST `online-update/cancel-downloading` with empty body.
#[test]
fn set_cancel_downloading_posts() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/online-update/cancel-downloading",
        "<response>OK</response>",
    );

    let ou = OnlineUpdate::new(&conn);
    ou.set_cancel_downloading().expect("ok");

    let body = tx.body_string_for("api/online-update/cancel-downloading");
    assert!(body.contains("<request>"), "body was: {body}");
    assert!(
        !body.contains("<cancel"),
        "expected no extra fields, body was: {body}"
    );
}

/// `online-update/upgrade-messagebox` resolves.
#[test]
fn upgrade_messagebox_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/online-update/upgrade-messagebox",
        "<response><Message>upgrade ready</Message></response>",
    );

    let ou = OnlineUpdate::new(&conn);
    let value = ou.upgrade_messagebox().expect("ok");
    assert_eq!(value["Message"], "upgrade ready");
}
