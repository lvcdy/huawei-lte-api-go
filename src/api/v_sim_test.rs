//! Tests for the [`VSim`] API group.

use crate::api::v_sim::VSim;
use crate::testsupport::conn_with;

/// `vsim/operateswitch-vsim` resolves.
#[test]
fn operateswitch_vsim_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/vsim/operateswitch-vsim",
        "<response><VSimSwitch>1</VSimSwitch></response>",
    );

    let vsim = VSim::new(&conn);
    let value = vsim.operateswitch_vsim().expect("ok");
    assert_eq!(value["VSimSwitch"], "1");
}
