//! Tests for the [`MLog`] API group.

use crate::api::m_log::MLog;
use crate::testsupport::conn_with;

/// `mlog/mobile-logger` resolves.
#[test]
fn mobile_logger_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/mlog/mobile-logger",
        "<response><MobileLogger>1</MobileLogger></response>",
    );

    let ml = MLog::new(&conn);
    let value = ml.mobile_logger().expect("ok");
    assert_eq!(value["MobileLogger"], "1");
}
