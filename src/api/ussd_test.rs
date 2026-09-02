//! Tests for the [`Ussd`] API group.

use crate::api::ussd::Ussd;
use crate::testsupport::conn_with;

/// `ussd/status` resolves.
#[test]
fn status_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/ussd/status", "<response><Status>0</Status></response>");

    let ussd = Ussd::new(&conn);
    let value = ussd.status().expect("ok");
    assert_eq!(value["Status"], "0");
}

/// `ussd/get` resolves.
#[test]
fn get_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/ussd/get",
        "<response><Content>*#06#</Content></response>",
    );

    let ussd = Ussd::new(&conn);
    let value = ussd.get().expect("ok");
    assert_eq!(value["Content"], "*#06#");
}

/// `ussd/send` posts content/codeType/timeout.
#[test]
fn send_posts_content() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/ussd/send",
        "<response><Content>reply</Content></response>",
    );

    let ussd = Ussd::new(&conn);
    let value = ussd.send("*100#").expect("ok");
    assert_eq!(value["Content"], "reply");

    let body = tx.body_string_for("api/ussd/send");
    assert!(
        body.contains("<content>*100#</content>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<codeType>codeType</codeType>"),
        "body was: {body}"
    );
}
