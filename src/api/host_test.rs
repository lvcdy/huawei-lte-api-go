//! Tests for the [`Host`] API group.

use crate::api::host::Host;
use crate::testsupport::conn_with;

/// `host/info` posts the expected fields.
#[test]
fn info_posts_all_fields() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/host/info", "<response>OK</response>");

    let host = Host::new(&conn);
    host.info(
        "20240601120000",
        "GMT+0200",
        "Linux x86_64",
        "Mozilla/5.0",
        "1.0.0",
    )
    .expect("ok");

    let body = tx.body_string_for("api/host/info");
    assert!(
        body.contains("<Time>20240601120000</Time>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<Timezone>GMT+0200</Timezone>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<Platform>Linux x86_64</Platform>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<PlatformVer>Mozilla/5.0</PlatformVer>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<Navigator>1.0.0</Navigator>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<NavigatorVer>Mozilla/5.0</NavigatorVer>"),
        "body was: {body}"
    );
}
