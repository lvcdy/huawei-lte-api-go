//! Tests for the [`Monitoring`] API group.

use crate::api::monitoring::Monitoring;
use crate::testsupport::conn_with;

/// GET endpoints resolve the `<response>` payload into JSON.
#[test]
fn status_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/monitoring/status",
        "<response><ConnectionStatus>901</ConnectionStatus><SignalIcon>3</SignalIcon></response>",
    );

    let mon = Monitoring::new(&conn);
    let value = mon.status().expect("status ok");

    assert_eq!(value["ConnectionStatus"], "901");
    assert_eq!(value["SignalIcon"], "3");
}

/// Traffic statistics nested payload.
#[test]
fn traffic_statistics_nested() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/monitoring/traffic-statistics",
        "<response><CurrentMonth><Download>1024</Download><Upload>512</Upload></CurrentMonth></response>",
    );

    let mon = Monitoring::new(&conn);
    let value = mon.traffic_statistics().expect("traffic ok");

    assert_eq!(value["CurrentMonth"]["Download"], "1024");
    assert_eq!(value["CurrentMonth"]["Upload"], "512");
}

/// `set_start_date` POSTs the usage-alarm body.
#[test]
fn set_start_date_posts_alarm_body() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/monitoring/start_date", "<response>OK</response>");

    let mon = Monitoring::new(&conn);
    let result = mon.set_start_date(1, "1GB", 90).expect("set ok");
    assert_eq!(result, "OK");

    let body = tx.body_string_for("api/monitoring/start_date");
    assert!(body.contains("<StartDay>1</StartDay>"), "body was: {body}");
    assert!(
        body.contains("<DataLimit>1GB</DataLimit>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<MonthThreshold>90</MonthThreshold>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<SetMonthData>1</SetMonthData>"),
        "body was: {body}"
    );
}

/// `set_clear_traffic` POSTs the clear flag.
#[test]
fn set_clear_traffic_posts_flag() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/monitoring/clear-traffic", "<response>OK</response>");

    let mon = Monitoring::new(&conn);
    mon.set_clear_traffic().expect("clear ok");

    let body = tx.body_string_for("api/monitoring/clear-traffic");
    assert!(
        body.contains("<ClearTraffic>1</ClearTraffic>"),
        "body was: {body}"
    );
}
