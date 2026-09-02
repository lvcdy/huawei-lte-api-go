//! Tests for the [`DDns`] API group.

use crate::api::d_dns::DDns;
use crate::testsupport::conn_with;

/// `ddns/ddns-list` resolves.
#[test]
fn get_ddns_list_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/ddns/ddns-list",
        "<response><Status>0</Status></response>",
    );

    let ddns = DDns::new(&conn);
    let value = ddns.get_ddns_list().expect("list ok");
    assert_eq!(value["Status"], "0");
}

/// `ddns/status` resolves.
#[test]
fn get_status_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/ddns/status",
        "<response><DdnsStatus>1</DdnsStatus></response>",
    );

    let ddns = DDns::new(&conn);
    let value = ddns.get_status().expect("status ok");
    assert_eq!(value["DdnsStatus"], "1");
}

/// `ddns/serverlist` resolves.
#[test]
fn serverlist_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/ddns/serverlist",
        "<response><ServerList><Item>dyndns.org</Item></ServerList></response>",
    );

    let ddns = DDns::new(&conn);
    let value = ddns.serverlist().expect("serverlist ok");
    assert_eq!(value["ServerList"]["Item"], "dyndns.org");
}
