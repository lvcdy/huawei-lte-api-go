//! Tests for the [`TimeRule`] API group.

use crate::api::time_rule::TimeRule;
use crate::testsupport::conn_with;

/// `timerule/timerule` resolves.
#[test]
fn timerule_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/timerule/timerule",
        "<response><TimeRuleCount>2</TimeRuleCount></response>",
    );

    let tr = TimeRule::new(&conn);
    let value = tr.timerule().expect("ok");
    assert_eq!(value["TimeRuleCount"], "2");
}
