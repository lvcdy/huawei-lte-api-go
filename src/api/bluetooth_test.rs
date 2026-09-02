//! Tests for the [`Bluetooth`] API group.

use crate::api::bluetooth::Bluetooth;
use crate::testsupport::conn_with;

/// `bluetooth/settings` resolves.
#[test]
fn settings_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/bluetooth/settings",
        "<response><BluetoothStatus>1</BluetoothStatus></response>",
    );

    let b = Bluetooth::new(&conn);
    let value = b.settings().expect("settings ok");
    assert_eq!(value["BluetoothStatus"], "1");
}

/// `bluetooth/scan` resolves.
#[test]
fn scan_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/bluetooth/scan",
        "<response><ScanState>0</ScanState></response>",
    );

    let b = Bluetooth::new(&conn);
    let value = b.scan().expect("scan ok");
    assert_eq!(value["ScanState"], "0");
}
