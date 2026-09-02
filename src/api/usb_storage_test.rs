//! Tests for the [`UsbStorage`] API group.

use crate::api::usb_storage::UsbStorage;
use crate::testsupport::conn_with;

/// `usbstorage/fsstatus` resolves.
#[test]
fn fsstatus_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/usbstorage/fsstatus",
        "<response><FsStatus>1</FsStatus></response>",
    );

    let us = UsbStorage::new(&conn);
    let value = us.fsstatus().expect("ok");
    assert_eq!(value["FsStatus"], "1");
}

/// `usbstorage/usbaccount` resolves.
#[test]
fn usbaccount_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/usbstorage/usbaccount",
        "<response><UsbAccount>account</UsbAccount></response>",
    );

    let us = UsbStorage::new(&conn);
    let value = us.usbaccount().expect("ok");
    assert_eq!(value["UsbAccount"], "account");
}
