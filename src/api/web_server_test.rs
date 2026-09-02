//! Tests for the [`WebServer`] API group.

use crate::api::web_server::WebServer;
use crate::testsupport::conn_with;

/// `webserver/publickey` resolves.
#[test]
fn publickey_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/webserver/publickey",
        "<response><PublicKey>key</PublicKey></response>",
    );

    let ws = WebServer::new(&conn);
    let value = ws.publickey().expect("ok");
    assert_eq!(value["PublicKey"], "key");
}

/// `webserver/token` resolves.
#[test]
fn token_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/webserver/token",
        "<response><Token>abc123</Token></response>",
    );

    let ws = WebServer::new(&conn);
    let value = ws.token().expect("ok");
    assert_eq!(value["Token"], "abc123");
}

/// `webserver/white_list_switch` resolves.
#[test]
fn white_list_switch_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/webserver/white_list_switch",
        "<response><WhiteListSwitch>1</WhiteListSwitch></response>",
    );

    let ws = WebServer::new(&conn);
    let value = ws.white_list_switch().expect("ok");
    assert_eq!(value["WhiteListSwitch"], "1");
}

/// `webserver/SesTokInfo` resolves.
#[test]
fn ses_tok_info_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/webserver/SesTokInfo",
        "<response><SesInfo>session</SesInfo><TokInfo>token</TokInfo></response>",
    );

    let ws = WebServer::new(&conn);
    let value = ws.ses_tok_info().expect("ok");
    assert_eq!(value["SesInfo"], "session");
    assert_eq!(value["TokInfo"], "token");
}
