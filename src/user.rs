//! User / authentication API group.
//!
//! Mirrors the Python library's `User.py`. The key method is
//! [`User::login`], which performs the login handshake and stores the
//! resulting [`UserSession`].

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine as _;
use sha2::{Digest, Sha256};

use crate::api::{get_value, post_set, post_set_refresh};
use crate::connection::Connection;
use crate::enums::user::{CurrentUserType, LoginState, PasswordType};
use crate::errors::{Error, Result};
use crate::tools::map_of;
use crate::xml::XmlMap;

/// Default username when none is supplied.
pub const DEFAULT_USERNAME: &str = "admin";

/// The current authenticated user.
#[derive(Debug, Clone)]
pub struct UserSession {
    /// Username used to log in.
    pub username: String,
    /// Current user type.
    pub user_type: CurrentUserType,
}

/// A single login state snapshot from `user/state-login`.
#[derive(Debug, Clone)]
pub struct UserStateInfo {
    /// Current login state.
    pub state: LoginState,
    /// Current user type.
    pub user_type: CurrentUserType,
    /// Login error code (`None` if none).
    pub error_code: Option<i64>,
    /// Login error message.
    pub error_message: String,
}

/// The User API group.
pub struct User<'a> {
    conn: &'a Connection,
}

impl<'a> User<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        User { conn }
    }

    /// `user/state-login`. Current login state.
    pub fn state_login(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/state-login")
    }

    /// `user/state-login` with retry on connection resets.
    ///
    /// Some models close the connection if login state is queried too soon
    /// after session setup; retry up to 5 times with a short backoff.
    fn state_login_with_retry(&self) -> Result<serde_json::Value> {
        let tries = 5;
        let mut last_err = None;
        for i in 0..tries {
            match self.state_login() {
                Ok(v) => return Ok(v),
                Err(e) => {
                    if e.is_connection_error() {
                        if i == tries - 1 {
                            return Err(e);
                        }
                        last_err = Some(e);
                        std::thread::sleep(std::time::Duration::from_millis((i + 1) * 100));
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        Err(Error::Unexpected(format!(
            "user/state-login retries exhausted: {last_err:?}"
        )))
    }

    /// Encode a password for the login request, mirroring the Python
    /// `_encode_password`: BASE64 of the password, or the SHA256-wrapped form
    /// when the device asks for it.
    fn encode_password(
        &self,
        username: &str,
        password: Option<&str>,
        password_type: PasswordType,
    ) -> Result<String> {
        let pwd = match password {
            Some(p) if !p.is_empty() => p,
            _ => return Ok(String::new()),
        };

        match password_type {
            PasswordType::Sha256 => {
                let token = self.conn.session().csrf_token().unwrap_or_default();
                // SHA256(password) -> hex string -> base64(hex).
                let hex1 = hex::encode(Sha256::digest(pwd.as_bytes()));
                let b64_hex = B64.encode(hex1.as_bytes());

                // username || b64(hex) || csrf-token
                let mut concentrated =
                    Vec::with_capacity(username.len() + b64_hex.len() + token.len());
                concentrated.extend_from_slice(username.as_bytes());
                concentrated.extend_from_slice(b64_hex.as_bytes());
                concentrated.extend_from_slice(token.as_bytes());

                // base64(SHA256(concentrated).hexdigest())
                let hex2 = hex::encode(Sha256::digest(&concentrated));
                Ok(B64.encode(hex2.as_bytes()))
            }
            _ => Ok(B64.encode(pwd.as_bytes())),
        }
    }

    /// Perform the actual login POST and return whether it succeeded.
    fn login_post(
        &self,
        username: &str,
        password: Option<&str>,
        password_type: PasswordType,
    ) -> Result<bool> {
        let encoded = self.encode_password(username, password, password_type)?;
        let body: XmlMap = map_of([
            ("Username", username.to_string()),
            ("Password", encoded),
            ("password_type", password_type.as_i32().to_string()),
        ]);
        let result = post_set_refresh(self.conn, "api/user/login", &body)?;
        if result == "OK" {
            self.conn.session().set_authenticated();
            self.conn.set_user(UserSession {
                username: username.to_string(),
                user_type: CurrentUserType::L2,
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// `user/login`. Log in, returning whether the login succeeded.
    ///
    /// * `username` — defaults to [`DEFAULT_USERNAME`] when empty or `""`.
    /// * `password` — `None`/empty means "don't send a password".
    /// * `force_new_login` — log in even if already logged in.
    pub fn login(
        &self,
        username: &str,
        password: Option<&str>,
        force_new_login: bool,
    ) -> Result<bool> {
        let username = if username.is_empty() {
            DEFAULT_USERNAME
        } else {
            username
        };

        let state = match self.state_login_with_retry() {
            Ok(v) => v,
            Err(Error::NotSupported { .. }) => return Ok(true),
            Err(e) => return Err(e),
        };

        let state_val = state.get("State").and_then(int_of).unwrap_or(0);
        if LoginState::from_i32(state_val) == LoginState::LoggedIn && !force_new_login {
            return Ok(true);
        }

        let password_type =
            PasswordType::from_i32(state.get("password_type").and_then(int_of).unwrap_or(0));
        self.login_post(username, password, password_type)
    }

    /// `user/logout`. Log out the current session.
    pub fn logout(&self) -> Result<String> {
        let body: XmlMap = map_of([("Logout", "1".to_string())]);
        let result = post_set(self.conn, "api/user/logout", &body)?;
        self.conn.session().clear_authenticated();
        self.conn.clear_user();
        Ok(result)
    }

    /// `user/remind`. Remind settings.
    pub fn remind(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/remind")
    }

    /// `user/password`. Password settings.
    pub fn password(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/password")
    }

    /// `user/pwd`. Current password info.
    pub fn pwd(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/pwd")
    }

    /// `user/pwd` (POST). Set the WLAN module password.
    pub fn set_pwd(&self) -> Result<String> {
        let body: XmlMap = map_of([
            ("module", "wlan".to_string()),
            ("nonce", "aaaaaaa".to_string()),
        ]);
        post_set(self.conn, "api/user/pwd", &body)
    }

    /// `user/remind` (POST). Set the remind state.
    pub fn set_remind(&self, remind_state: &str) -> Result<String> {
        let body: XmlMap = map_of([("remindstate", remind_state.to_string())]);
        post_set(self.conn, "api/user/remind", &body)
    }

    /// `user/authentication_login`. Authentication login info.
    pub fn authentication_login(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/authentication_login")
    }

    /// `user/challenge_login`. Challenge login info.
    pub fn challenge_login(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/challenge_login")
    }

    /// `user/hilink_login`. HiLink login info.
    pub fn hilink_login(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/hilink_login")
    }

    /// `user/history-login`. History login info.
    pub fn history_login(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/history-login")
    }

    /// `user/heartbeat`. Session heartbeat.
    pub fn heartbeat(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/heartbeat")
    }

    /// `user/web-feature-switch`. Web feature switch status.
    pub fn web_feature_switch(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/web-feature-switch")
    }

    /// `user/input_event`. Reverse-engineered endpoint, unknown usage.
    pub fn input_event(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/input_event")
    }

    /// `user/screen_state`. Reverse-engineered endpoint, unknown usage.
    pub fn screen_state(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/screen_state")
    }

    /// `user/session`. Reverse-engineered endpoint, unknown usage.
    pub fn session(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/session")
    }

    /// `user/second_login`. Second login info.
    pub fn second_login(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/second_login")
    }

    /// `user/remember-pwd`. Remember-password status.
    pub fn remember_pwd(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/remember-pwd")
    }

    /// `user/rule`. User rule info.
    pub fn rule(&self) -> Result<serde_json::Value> {
        get_value(self.conn, "api/user/rule")
    }
}

/// Read a JSON value as `i32`, handling both numbers and numeric strings.
fn int_of(v: &serde_json::Value) -> Option<i32> {
    v.as_i64()
        .and_then(|i| i32::try_from(i).ok())
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
}
