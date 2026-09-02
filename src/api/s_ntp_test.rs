//! Tests for the [`SNtp`] API group.

use crate::api::s_ntp::SNtp;
use crate::testsupport::conn_with;

/// `sntp/settings` resolves.
#[test]
fn get_settings_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/sntp/settings",
        "<response><SntpSwitch>1</SntpSwitch></response>",
    );

    let sntp = SNtp::new(&conn);
    let value = sntp.get_settings().expect("ok");
    assert_eq!(value["SntpSwitch"], "1");
}

/// `sntp/sntpswitch` resolves.
#[test]
fn sntpswitch_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/sntp/sntpswitch",
        "<response><SntpSwitch>0</SntpSwitch></response>",
    );

    let sntp = SNtp::new(&conn);
    let value = sntp.sntpswitch().expect("ok");
    assert_eq!(value["SntpSwitch"], "0");
}

/// `sntp/serverinfo` resolves.
#[test]
fn serverinfo_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/sntp/serverinfo",
        "<response><Server>pool.ntp.org</Server></response>",
    );

    let sntp = SNtp::new(&conn);
    let value = sntp.serverinfo().expect("ok");
    assert_eq!(value["Server"], "pool.ntp.org");
}

/// `sntp/timeinfo` resolves.
#[test]
fn timeinfo_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/sntp/timeinfo",
        "<response><Time>2024-06-01 12:00:00</Time></response>",
    );

    let sntp = SNtp::new(&conn);
    let value = sntp.timeinfo().expect("ok");
    assert_eq!(value["Time"], "2024-06-01 12:00:00");
}
