//! Tests for the [`Device`] API group.
//!
//! These exercise the migrated API group through a fake [`HttpTransport`],
//! verifying endpoint paths, HTTP methods, CSRF tokens, request bodies and
//! response/error mapping.

use crate::enums::device::{ControlMode, Mode};
use crate::testsupport::{conn_with, FakeResponse};

use crate::api::device::Device;

/// GET endpoints resolve the response payload into a JSON value.
#[test]
fn device_information_get_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/device/information",
        "<response><DeviceName>B818</DeviceName><IMEI>864156032481234</IMEI></response>",
    );

    let dev = Device::new(&conn);
    let value = dev.information().expect("information ok");

    assert_eq!(value["DeviceName"], "B818");
    assert_eq!(value["IMEI"], "864156032481234");

    // The GET carried the CSRF token discovered at session init.
    let req = tx
        .requests()
        .into_iter()
        .find(|r| r.url.contains("api/device/information"))
        .expect("device/information request recorded");
    assert_eq!(req.token.as_deref(), Some("CSRF_TOKEN_HOME"));
}

/// A GET with no `<response>` wrapper still returns the payload map.
#[test]
fn get_without_response_wrapper_falls_back_to_whole_map() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/device/signal",
        "<Signal><rsrp>-97</rsrp><sinr>12</sinr></Signal>",
    );

    let dev = Device::new(&conn);
    let value = dev.signal().expect("signal ok");

    // No `<response>` wrapper: the whole map (rooted at the document element)
    // is returned, so the payload sits under the `Signal` key.
    assert_eq!(value["Signal"]["rsrp"], "-97");
    assert_eq!(value["Signal"]["sinr"], "12");
}

/// `set_control` POSTs the right body and returns the response text.
#[test]
fn set_control_posts_control_body() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/device/control", "<response>OK</response>");

    let dev = Device::new(&conn);
    let result = dev.set_control(ControlMode::Reboot).expect("control ok");

    assert_eq!(result, "OK");

    let body = tx.body_string_for("api/device/control");
    assert!(body.contains("<Control>1</Control>"), "body was: {body}");
}

/// `mode()` uses the `Mode` enum's integer value.
#[test]
fn mode_posts_enum_value() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/device/mode", "<response>OK</response>");

    let dev = Device::new(&conn);
    dev.mode(Mode::EnableTelnet).expect("mode ok");

    let body = tx.body_string_for("api/device/mode");
    assert!(body.contains("<mode>2</mode>"), "body was: {body}");
}

/// Boolean flags are serialised as `"1"`/`"0"`.
#[test]
fn set_basic_information_serialises_boolean() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/device/basic_information", "<response>OK</response>");

    let dev = Device::new(&conn);
    dev.set_basic_information(true).expect("set ok");

    let body = tx.body_string_for("api/device/basic_information");
    assert!(
        body.contains("<restore_default_status>1</restore_default_status>"),
        "body was: {body}"
    );
}

/// Device errors are mapped onto typed `Error::NotSupported`.
#[test]
fn error_response_maps_to_not_supported() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/device/control",
        "<error><code>100002</code><message>No support</message></error>",
    );

    let dev = Device::new(&conn);
    let err = dev
        .set_control(ControlMode::Reset)
        .expect_err("should fail");

    match err {
        crate::Error::NotSupported { code, message } => {
            assert_eq!(code, 100002);
            assert_eq!(message, "No support");
        }
        other => panic!("expected NotSupported, got {other:?}"),
    }
}

/// A CSRF-invalidated session reloads once and retries (mirroring the Python
/// retry-on-LoginCsrf behaviour).
///
/// The fake transport always returns the CSRF error for this endpoint, so the
/// first call fails, `get_xml` reloads (re-fetching the homepage) and issues a
/// second request. We assert on the side effects (request counts).
#[test]
fn csrf_failure_triggers_reload_and_retry() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/device/information",
        "<error><code>125002</code><message>Session error</message></error>",
    );

    let dev = Device::new(&conn);
    let err = dev.information().expect_err("should fail");
    assert!(matches!(err, crate::Error::LoginCsrf { .. }));

    let requests = tx.requests();
    let info_hits = requests
        .iter()
        .filter(|r| r.url.contains("api/device/information"))
        .count();
    let home_hits = requests
        .iter()
        .filter(|r| r.url.ends_with("cpe.local/"))
        .count();
    assert!(info_hits >= 2, "expected retry, saw {info_hits} hits");
    assert!(home_hits >= 2, "expected reload, saw {home_hits} hits");
}

/// 5G supplementary GET endpoints (Brovi) resolve correctly.
#[test]
fn five_g_cell_info_endpoints() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/device/seccellinfo",
        "<response><cellId>123</cellId><freq>633280</freq></response>",
    );
    tx.route_xml(
        "api/device/nbrcellinfo",
        "<response><NbrCell><pci>42</pci></NbrCell></response>",
    );

    let dev = Device::new(&conn);

    let sec = dev.get_sec_cell_info().expect("seccellinfo ok");
    assert_eq!(sec["cellId"], "123");

    let nbr = dev.get_nbr_cell_info().expect("nbrcellinfo ok");
    assert_eq!(nbr["NbrCell"]["pci"], "42");
}

/// JSON responses (Content-Type: application/json) are decoded too.
#[test]
fn json_response_is_decoded() {
    let (conn, tx) = conn_with();
    tx.route(
        "api/device/boot_time",
        FakeResponse::json(r#"{"response":{"BootTime":"2026-01-01 00:00:00"}}"#),
    );

    let dev = Device::new(&conn);
    let value = dev.boot_time().expect("boot_time ok");
    assert_eq!(value["BootTime"], "2026-01-01 00:00:00");
}
