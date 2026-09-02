//! Tests for the [`Time`] API group.

use crate::api::time::Time;
use crate::testsupport::conn_with;

/// `time/timeout` resolves.
#[test]
fn timeout_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/time/timeout",
        "<response><Timeout>600</Timeout></response>",
    );

    let time = Time::new(&conn);
    let value = time.timeout().expect("ok");
    assert_eq!(value["Timeout"], "600");
}
