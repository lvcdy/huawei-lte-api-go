//! Tests for the [`Global`] API group.

use crate::api::global::Global;
use crate::testsupport::conn_with;

/// Each GET endpoint resolves into JSON.
#[test]
fn module_switch_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/global/module-switch",
        "<response><Sms>1</Sms><Filemanage>1</Filemanage><Wlan>1</Wlan></response>",
    );

    let g = Global::new(&conn);
    let value = g.module_switch().expect("module switch ok");

    assert_eq!(value["Sms"], "1");
    assert_eq!(value["Wlan"], "1");
}

/// `custommenu-url` resolves.
#[test]
fn custommenu_url_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/global/custommenu-url",
        "<response><url>http://192.168.8.1/html/menu.html</url></response>",
    );

    let g = Global::new(&conn);
    let value = g.custommenu_url().expect("custommenu ok");
    assert_eq!(value["url"], "http://192.168.8.1/html/menu.html");
}
