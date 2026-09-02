//! Tests for the [`Ntwk`] API group.

use crate::api::ntwk::Ntwk;
use crate::testsupport::conn_with;

/// `ntwk/lan_upnp_portmapping` resolves.
#[test]
fn lan_upnp_portmapping_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/ntwk/lan_upnp_portmapping",
        "<response><MappingCount>2</MappingCount></response>",
    );

    let ntwk = Ntwk::new(&conn);
    let value = ntwk.lan_upnp_portmapping().expect("ok");
    assert_eq!(value["MappingCount"], "2");
}

/// `ntwk/celllock` resolves.
#[test]
fn celllock_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/ntwk/celllock",
        "<response><CellLockState>0</CellLockState></response>",
    );

    let ntwk = Ntwk::new(&conn);
    let value = ntwk.celllock().expect("ok");
    assert_eq!(value["CellLockState"], "0");
}

/// `ntwk/dualwaninfo` resolves.
#[test]
fn dualwaninfo_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/ntwk/dualwaninfo",
        "<response><DualWanState>1</DualWanState></response>",
    );

    let ntwk = Ntwk::new(&conn);
    let value = ntwk.dualwaninfo().expect("ok");
    assert_eq!(value["DualWanState"], "1");
}

/// `ntwk/lan-wan-config` resolves.
#[test]
fn lan_wan_config_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/ntwk/lan-wan-config",
        "<response><LanWanConfig>cfg</LanWanConfig></response>",
    );

    let ntwk = Ntwk::new(&conn);
    let value = ntwk.lan_wan_config().expect("ok");
    assert_eq!(value["LanWanConfig"], "cfg");
}
