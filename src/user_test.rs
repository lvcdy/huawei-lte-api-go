//! Tests for the [`User`] API group (login handshake, logout, GET endpoints).

use crate::testsupport::conn_with;
use crate::user::User;

/// A fresh session (State=logged-out) performs the login POST and reports
/// success when the device answers `OK`.
#[test]
fn login_posts_when_logged_out() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/user/state-login",
        "<response><State>-1</State><password_type>0</password_type></response>",
    );
    tx.route_xml("api/user/login", "<response>OK</response>");

    let user = User::new(&conn);
    let ok = user
        .login("admin", Some("password"), false)
        .expect("login ok");
    assert!(ok);

    // The login POST carried the encoded password and the CSRF token.
    let body = tx.body_string_for("api/user/login");
    assert!(
        body.contains("<Username>admin</Username>"),
        "body was: {body}"
    );
    assert!(body.contains("<Password>"), "body was: {body}");
    assert!(
        body.contains("<password_type>0</password_type>"),
        "body was: {body}"
    );

    // Session is now marked authenticated.
    assert!(
        conn.session().is_authenticated(),
        "session should be authenticated"
    );
}

/// An already-logged-in session short-circuits without a login POST.
#[test]
fn login_skips_when_already_logged_in() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/user/state-login",
        "<response><State>0</State><password_type>0</password_type></response>",
    );

    let user = User::new(&conn);
    let ok = user
        .login("admin", Some("password"), false)
        .expect("login ok");
    assert!(ok);

    // No login POST should have been issued.
    let login_hits = tx
        .requests()
        .into_iter()
        .filter(|r| r.url.contains("api/user/login"))
        .count();
    assert_eq!(
        login_hits, 0,
        "expected no login POST when already logged in"
    );
}

/// `force_new_login` re-authenticates even when already logged in.
#[test]
fn login_forces_when_requested() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/user/state-login",
        "<response><State>0</State><password_type>0</password_type></response>",
    );
    tx.route_xml("api/user/login", "<response>OK</response>");

    let user = User::new(&conn);
    let ok = user
        .login("admin", Some("password"), true)
        .expect("login ok");
    assert!(ok);

    let login_hits = tx
        .requests()
        .into_iter()
        .filter(|r| r.url.contains("api/user/login"))
        .count();
    assert_eq!(login_hits, 1, "expected a login POST when forced");
}

/// A non-`OK` login response reports failure and leaves the session
/// unauthenticated.
#[test]
fn login_failure_reports_false() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/user/state-login",
        "<response><State>-1</State><password_type>0</password_type></response>",
    );
    tx.route_xml("api/user/login", "<response>Error</response>");

    let user = User::new(&conn);
    let ok = user.login("admin", Some("wrong"), false).expect("login ok");
    assert!(!ok);
    assert!(!conn.session().is_authenticated());
}

/// `logout` POSTs the logout body and clears the authenticated state.
#[test]
fn logout_posts_and_clears() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/user/logout", "<response>OK</response>");

    // Simulate an authenticated session.
    conn.session().set_authenticated();
    conn.set_user(crate::user::UserSession {
        username: "admin".to_string(),
        user_type: crate::enums::user::CurrentUserType::L2,
    });

    let user = User::new(&conn);
    let result = user.logout().expect("logout ok");
    assert_eq!(result, "OK");
    assert!(
        !conn.session().is_authenticated(),
        "session should be cleared"
    );

    let body = tx.body_string_for("api/user/logout");
    assert!(body.contains("<Logout>1</Logout>"), "body was: {body}");
}

/// GET endpoints resolve into JSON.
#[test]
fn heartbeat_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/user/heartbeat",
        "<response><Heartbeat>1</Heartbeat></response>",
    );

    let user = User::new(&conn);
    let value = user.heartbeat().expect("heartbeat ok");
    assert_eq!(value["Heartbeat"], "1");
}
