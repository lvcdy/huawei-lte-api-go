//! Tests for the [`Sms`] API group.

use crate::api::sms::Sms;
use crate::testsupport::conn_with;

/// GET endpoints resolve into JSON.
#[test]
fn sms_count_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/sms/sms-count",
        "<response><LocalInbox>5</LocalInbox><LocalDraftbox>1</LocalDraftbox></response>",
    );

    let sms = Sms::new(&conn);
    let value = sms.sms_count().expect("count ok");

    assert_eq!(value["LocalInbox"], "5");
    assert_eq!(value["LocalDraftbox"], "1");
}

/// `get_sms_list` POSTs a full paging body and returns a list payload.
#[test]
fn get_sms_list_posts_paging_body() {
    let (conn, tx) = conn_with();
    // Two `<Message>` siblings fold into a JSON array under `Messages`.
    tx.route_xml(
        "api/sms/sms-list",
        "<response><Messages><Message><Index>1</Index><Content>hello</Content></Message><Message><Index>2</Index><Content>world</Content></Message></Messages></response>",
    );

    let sms = Sms::new(&conn);
    let value = sms.get_sms_list(1, 1, 20, 0, true, false).expect("list ok");

    assert_eq!(value["Messages"]["Message"][0]["Index"], "1");
    assert_eq!(value["Messages"]["Message"][0]["Content"], "hello");
    assert_eq!(value["Messages"]["Message"][1]["Index"], "2");

    let body = tx.body_string_for("api/sms/sms-list");
    assert!(
        body.contains("<PageIndex>1</PageIndex>"),
        "body was: {body}"
    );
    assert!(body.contains("<BoxType>1</BoxType>"), "body was: {body}");
    assert!(
        body.contains("<ReadCount>20</ReadCount>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<Ascending>1</Ascending>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<UnreadPreferred>0</UnreadPreferred>"),
        "body was: {body}"
    );
}

/// `delete_sms` POSTs the index.
#[test]
fn delete_sms_posts_index() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/sms/delete-sms", "<response>OK</response>");

    let sms = Sms::new(&conn);
    sms.delete_sms(42).expect("delete ok");

    let body = tx.body_string_for("api/sms/delete-sms");
    assert!(body.contains("<Index>42</Index>"), "body was: {body}");
}

/// `send_sms` builds the full Phones/Content body.
#[test]
fn send_sms_builds_full_body() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/sms/send-sms", "<response>OK</response>");

    let sms = Sms::new(&conn);
    let phones = vec!["+8613800138000".to_string(), "+8613900139000".to_string()];
    sms.send_sms(
        &phones,
        "hi there",
        -1,
        Some("+8613800138000"),
        0,
        "2026-01-01 00:00:00",
    )
    .expect("send ok");

    let body = tx.body_string_for("api/sms/send-sms");
    assert!(body.contains("<Phones>"), "body was: {body}");
    assert!(
        body.contains("<Phone>+8613800138000</Phone>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<Phone>+8613900139000</Phone>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<Content>hi there</Content>"),
        "body was: {body}"
    );
    assert!(body.contains("<Index>-1</Index>"), "body was: {body}");
    assert!(body.contains("<Length>8</Length>"), "body was: {body}");
}

/// `cancel_send` issues a POST to the cancel endpoint.
#[test]
fn cancel_send_posts() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/sms/cancel-send", "<response>OK</response>");

    let sms = Sms::new(&conn);
    sms.cancel_send().expect("cancel ok");

    let hits = tx
        .requests()
        .into_iter()
        .filter(|r| r.url.contains("api/sms/cancel-send"))
        .count();
    assert_eq!(hits, 1, "expected a single cancel-send POST");
}
