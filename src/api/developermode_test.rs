//! Tests for the [`Developermode`] API group.

use crate::api::developermode::Developermode;
use crate::testsupport::conn_with;

/// `developermode/developer-mode` resolves.
#[test]
fn developer_mode_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/developermode/developer-mode",
        "<response><DevMode>1</DevMode></response>",
    );

    let dm = Developermode::new(&conn);
    let value = dm.developer_mode().expect("ok");
    assert_eq!(value["DevMode"], "1");
}

/// `developermode/developer-item` resolves.
#[test]
fn developer_item_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/developermode/developer-item",
        "<response><Item>1</Item></response>",
    );

    let dm = Developermode::new(&conn);
    let value = dm.developer_item().expect("ok");
    assert_eq!(value["Item"], "1");
}
