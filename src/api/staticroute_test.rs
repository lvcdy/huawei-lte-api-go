//! Tests for the [`Staticroute`] API group.

use crate::api::staticroute::Staticroute;
use crate::testsupport::conn_with;

/// `staticroute/wanpath` resolves.
#[test]
fn wanpath_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/staticroute/wanpath",
        "<response><WanPath>1</WanPath></response>",
    );

    let sr = Staticroute::new(&conn);
    let value = sr.wanpath().expect("ok");
    assert_eq!(value["WanPath"], "1");
}
