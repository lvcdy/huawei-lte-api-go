//! Tests for the [`Pb`] API group.

use crate::api::pb::Pb;
use crate::testsupport::conn_with;

/// `pb/pb-match` posts the phone number.
#[test]
fn get_pb_match_posts_phone() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/pb/pb-match", "<response><Match>1</Match></response>");

    let pb = Pb::new(&conn);
    let value = pb.get_pb_match("+8613800138000").expect("ok");
    assert_eq!(value["Match"], "1");

    let body = tx.body_string_for("api/pb/pb-match");
    assert!(
        body.contains("<Phone>+8613800138000</Phone>"),
        "body was: {body}"
    );
}

/// `pb/pb-list` posts paging + sort fields.
#[test]
fn get_pb_list_posts_paging_fields() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/pb/pb-list", "<response><Count>5</Count></response>");

    let pb = Pb::new(&conn);
    let value = pb.get_pb_list(1, "bob", 0, 50, 0, 1, 0).expect("ok");
    assert_eq!(value["Count"], "5");

    let body = tx.body_string_for("api/pb/pb-list");
    assert!(body.contains("<GroupID>0</GroupID>"), "body was: {body}");
    assert!(
        body.contains("<PageIndex>1</PageIndex>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<ReadCount>50</ReadCount>"),
        "body was: {body}"
    );
    assert!(body.contains("<SaveType>0</SaveType>"), "body was: {body}");
    assert!(body.contains("<SortType>1</SortType>"), "body was: {body}");
    assert!(
        body.contains("<Ascending>0</Ascending>"),
        "body was: {body}"
    );
    assert!(body.contains("<KeyWord>bob</KeyWord>"), "body was: {body}");
}

/// `pb/pb-count` posts with an empty body.
#[test]
fn pb_count_posts_empty() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/pb/pb-count", "<response><Count>42</Count></response>");

    let pb = Pb::new(&conn);
    let value = pb.pb_count().expect("ok");
    assert_eq!(value["Count"], "42");
}

/// `pb/group-count` posts with an empty body.
#[test]
fn group_count_posts_empty() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/pb/group-count",
        "<response><Count>3</Count></response>",
    );

    let pb = Pb::new(&conn);
    let value = pb.group_count().expect("ok");
    assert_eq!(value["Count"], "3");
}

/// `pb/pb-new` wraps fields in a `<Field>` list.
#[test]
fn pb_new_wraps_fields_in_list() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/pb/pb-new", "<response>OK</response>");

    let pb = Pb::new(&conn);
    pb.pb_new(0, 0, "Bob", "+8613800138000", "", "", "bob@example.com")
        .expect("ok");

    let body = tx.body_string_for("api/pb/pb-new");
    assert!(body.contains("<GroupID>0</GroupID>"), "body was: {body}");
    assert!(body.contains("<SaveType>0</SaveType>"), "body was: {body}");
    assert!(
        body.contains("<Name>FormattedName</Name>"),
        "body was: {body}"
    );
    assert!(body.contains("<Value>Bob</Value>"), "body was: {body}");
    assert!(
        body.contains("<Name>MobilePhone</Name>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<Value>+8613800138000</Value>"),
        "body was: {body}"
    );
    assert!(body.contains("<Name>WorkEmail</Name>"), "body was: {body}");
    assert!(
        body.contains("<Value>bob@example.com</Value>"),
        "body was: {body}"
    );
}

/// `pb/pb-delete` posts the index.
#[test]
fn pb_delete_posts_index() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/pb/pb-delete", "<response>OK</response>");

    let pb = Pb::new(&conn);
    pb.pb_delete(7).expect("ok");

    let body = tx.body_string_for("api/pb/pb-delete");
    assert!(body.contains("<Index>7</Index>"), "body was: {body}");
}

/// `pb/group-delete` posts the group id.
#[test]
fn group_delete_posts_group_id() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/pb/group-delete", "<response>OK</response>");

    let pb = Pb::new(&conn);
    pb.group_delete(2).expect("ok");

    let body = tx.body_string_for("api/pb/group-delete");
    assert!(body.contains("<GroupID>2</GroupID>"), "body was: {body}");
}
