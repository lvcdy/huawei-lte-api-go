//! Tests for the [`Language`] API group.

use crate::api::language::Language;
use crate::testsupport::conn_with;

/// `language/current-language` resolves.
#[test]
fn current_language_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/language/current-language",
        "<response><CurrentLanguage>en_us</CurrentLanguage></response>",
    );

    let lang = Language::new(&conn);
    let value = lang.current_language().expect("ok");
    assert_eq!(value["CurrentLanguage"], "en_us");
}

/// `set_current_language` posts the language code.
#[test]
fn set_current_language_posts_code() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/language/current-language", "<response>OK</response>");

    let lang = Language::new(&conn);
    lang.set_current_language("zh_cn").expect("ok");

    let body = tx.body_string_for("api/language/current-language");
    assert!(
        body.contains("<CurrentLanguage>zh_cn</CurrentLanguage>"),
        "body was: {body}"
    );
}
