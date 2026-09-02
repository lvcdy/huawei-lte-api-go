//! Tests for the [`Ota`] API group.

use crate::api::ota::Ota;
use crate::testsupport::conn_with;

/// `ota/status` resolves.
#[test]
fn status_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/ota/status",
        "<response><OtaStatus>1</OtaStatus></response>",
    );

    let ota = Ota::new(&conn);
    let value = ota.status().expect("ok");
    assert_eq!(value["OtaStatus"], "1");
}
