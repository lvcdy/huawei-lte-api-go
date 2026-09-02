//! Tests for the [`SdCard`] API group.

use crate::api::sd_card::SdCard;
use crate::testsupport::conn_with;

/// `sdcard/dlna-setting` resolves.
#[test]
fn dlna_setting_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/sdcard/dlna-setting",
        "<response><DlnaEnabled>1</DlnaEnabled></response>",
    );

    let sd = SdCard::new(&conn);
    let value = sd.dlna_setting().expect("ok");
    assert_eq!(value["DlnaEnabled"], "1");
}

/// `set_dlna_setting` posts enable/share flags.
#[test]
fn set_dlna_setting_posts_flags() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/sdcard/dlna-setting", "<response>OK</response>");

    let sd = SdCard::new(&conn);
    sd.set_dlna_setting(true, true, "/").expect("ok");

    let body = tx.body_string_for("api/sdcard/dlna-setting");
    assert!(body.contains("<enabled>1</enabled>"), "body was: {body}");
    assert!(
        body.contains("<sharepath>/</sharepath>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<shareallpath>1</shareallpath>"),
        "body was: {body}"
    );
}

/// `sdcard/sdcard` resolves.
#[test]
fn sdcard_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/sdcard/sdcard",
        "<response><SdStatus>1</SdStatus></response>",
    );

    let sd = SdCard::new(&conn);
    let value = sd.sdcard().expect("ok");
    assert_eq!(value["SdStatus"], "1");
}

/// `sdcard/sdcardsamba` resolves.
#[test]
fn sdcardsamba_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/sdcard/sdcardsamba",
        "<response><SambaEnabled>1</SambaEnabled></response>",
    );

    let sd = SdCard::new(&conn);
    let value = sd.sdcardsamba().expect("ok");
    assert_eq!(value["SambaEnabled"], "1");
}

/// `set_sdcardsamba` posts all SMB fields.
#[test]
fn set_sdcardsamba_posts_all_fields() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/sdcard/sdcardsamba", "<response>OK</response>");

    let sd = SdCard::new(&conn);
    sd.set_sdcardsamba(true, "HUAWEI", "Huawei CPE", "WORKGROUP", true, false)
        .expect("ok");

    let body = tx.body_string_for("api/sdcard/sdcardsamba");
    assert!(body.contains("<enabled>1</enabled>"), "body was: {body}");
    assert!(
        body.contains("<servername>HUAWEI</servername>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<serverdescription>Huawei CPE</serverdescription>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<workgroupname>WORKGROUP</workgroupname>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<anonymousaccess>1</anonymousaccess>"),
        "body was: {body}"
    );
    assert!(
        body.contains("<printerenable>0</printerenable>"),
        "body was: {body}"
    );
}

/// `sdcard/printerlist` resolves.
#[test]
fn printerlist_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/sdcard/printerlist",
        "<response><Printers><Printer><Name>p1</Name></Printer></Printers></response>",
    );

    let sd = SdCard::new(&conn);
    let value = sd.printerlist().expect("ok");
    assert_eq!(value["Printers"]["Printer"]["Name"], "p1");
}
