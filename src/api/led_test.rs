//! Tests for the [`Led`] API group.

use crate::api::led::Led;
use crate::testsupport::conn_with;

/// `led/nightmode` resolves.
#[test]
fn nightmode_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/led/nightmode",
        "<response><NightMode>1</NightMode></response>",
    );

    let led = Led::new(&conn);
    let value = led.nightmode().expect("nightmode ok");
    assert_eq!(value["NightMode"], "1");
}

/// `led/appctrlled` resolves.
#[test]
fn appctrlled_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/led/appctrlled",
        "<response><AppCtrlLed>0</AppCtrlLed></response>",
    );

    let led = Led::new(&conn);
    let value = led.appctrlled().expect("appctrlled ok");
    assert_eq!(value["AppCtrlLed"], "0");
}
