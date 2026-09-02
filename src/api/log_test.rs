//! Tests for the [`Log`] API group.

use crate::api::log::Log;
use crate::testsupport::conn_with;

/// `log/loginfo` resolves.
#[test]
fn loginfo_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/log/loginfo",
        "<response><LogInfo>some log</LogInfo></response>",
    );

    let log = Log::new(&conn);
    let value = log.loginfo().expect("ok");
    assert_eq!(value["LogInfo"], "some log");
}
