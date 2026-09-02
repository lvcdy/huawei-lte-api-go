//! Tests for the [`Cwmp`] API group.

use crate::api::cwmp::Cwmp;
use crate::testsupport::conn_with;

/// `cwmp/basic-info` resolves.
#[test]
fn basic_info_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/cwmp/basic-info",
        "<response><AcsUrl>http://acs.example</AcsUrl></response>",
    );

    let cwmp = Cwmp::new(&conn);
    let value = cwmp.basic_info().expect("basic info ok");
    assert_eq!(value["AcsUrl"], "http://acs.example");
}
