//! Tests for the [`Syslog`] API group.

use crate::api::syslog::Syslog;
use crate::testsupport::conn_with;

/// `syslog/querylog` resolves.
#[test]
fn querylog_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/syslog/querylog",
        "<response><Log>entry</Log></response>",
    );

    let syslog = Syslog::new(&conn);
    let value = syslog.querylog().expect("ok");
    assert_eq!(value["Log"], "entry");
}

/// `clear` posts the process-log clear command.
#[test]
fn clear_posts_command() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/syslog/processlog", "<response>OK</response>");

    let syslog = Syslog::new(&conn);
    syslog.clear().expect("ok");

    let body = tx.body_string_for("api/syslog/processlog");
    assert!(
        body.contains("<command>clear</command>"),
        "body was: {body}"
    );
}
