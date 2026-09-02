//! Tests for the [`Redirection`] API group.

use crate::api::redirection::Redirection;
use crate::testsupport::conn_with;

/// `redirection/homepage` resolves.
#[test]
fn homepage_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/redirection/homepage",
        "<response><Url>http://192.168.8.1/html/index.html</Url></response>",
    );

    let redir = Redirection::new(&conn);
    let value = redir.homepage().expect("ok");
    assert_eq!(value["Url"], "http://192.168.8.1/html/index.html");
}
