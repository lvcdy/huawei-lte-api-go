//! Tests for the [`Lan`] API group.

use crate::api::lan::Lan;
use crate::testsupport::conn_with;

/// `lan/HostInfo` resolves the host list (repeated `<Host>` → array).
#[test]
fn host_info_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/lan/HostInfo",
        "<response><Hosts><Host><IpAddress>192.168.8.100</IpAddress><MacAddress>AA:BB:CC:DD:EE:FF</MacAddress></Host><Host><IpAddress>192.168.8.101</IpAddress><MacAddress>11:22:33:44:55:66</MacAddress></Host></Hosts></response>",
    );

    let lan = Lan::new(&conn);
    let value = lan.host_info().expect("host info ok");
    assert_eq!(value["Hosts"]["Host"][0]["IpAddress"], "192.168.8.100");
    assert_eq!(value["Hosts"]["Host"][0]["MacAddress"], "AA:BB:CC:DD:EE:FF");
    assert_eq!(value["Hosts"]["Host"][1]["IpAddress"], "192.168.8.101");
}
