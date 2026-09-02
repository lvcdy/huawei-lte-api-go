//! Error types for the library.
//!
//! Mirrors the exception hierarchy of the Python `huawei-lte-api`:
//! all response errors carry an error `code` (the Huawei error code)
//! and a human-readable `message`.

/// Convenient crate-wide result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Huawei device response/error codes.
///
/// These mirror `ResponseCodeEnum` in the Python library.
pub mod codes {
    pub const ERROR_SYSTEM_UNKNOWN: i64 = 100001;
    pub const ERROR_SYSTEM_NO_SUPPORT: i64 = 100002;
    pub const ERROR_SYSTEM_NO_RIGHTS: i64 = 100003;
    pub const ERROR_SYSTEM_BUSY: i64 = 100004;
    pub const ERROR_FORMAT_ERROR: i64 = 100005;
    pub const ERROR_VOICE_BUSY: i64 = 120001; // Unused
    pub const ERROR_WRONG_TOKEN: i64 = 125001; // Unused
    pub const ERROR_SYSTEM_CSRF: i64 = 125002;
    pub const ERROR_WRONG_SESSION_TOKEN: i64 = 125003;
}

/// Root error type for the whole crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A response was returned from the device but it contained an error code.
    #[error("{message}")]
    Response {
        /// Error code returned by the device.
        code: i64,
        /// Human readable message.
        message: String,
    },

    /// The device does not support the requested feature/endpoint.
    #[error("{message}")]
    NotSupported { code: i64, message: String },

    /// The device returned a "login required" (no rights) error.
    #[error("{message}")]
    LoginRequired { code: i64, message: String },

    /// The device is busy.
    #[error("{message}")]
    SystemBusy { code: i64, message: String },

    /// CSRF/session token mismatch (device wants us to re-login).
    #[error("{message}")]
    LoginCsrf { code: i64, message: String },

    /// Wrong session token.
    #[error("{message}")]
    WrongSessionToken { code: i64, message: String },

    /// Request format error.
    #[error("{message}")]
    RequestFormat { code: i64, message: String },

    /// Login failed due to permanently invalid credentials.
    #[error("invalid credentials: {message}")]
    InvalidCredentials { code: i64, message: String },

    /// Login failed because the username is wrong.
    #[error("username wrong: {message}")]
    UsernameWrong { code: i64, message: String },

    /// Login failed because the password is wrong.
    #[error("password wrong: {message}")]
    PasswordWrong { code: i64, message: String },

    /// Already logged in (and not asked to force a new login).
    #[error("already logged in: {message}")]
    AlreadyLogin { code: i64, message: String },

    /// Username and password both wrong.
    #[error("username and password wrong: {message}")]
    UsernamePasswordWrong { code: i64, message: String },

    /// Login attempt overrun (too many attempts).
    #[error("login attempt overrun: {message}")]
    UsernamePasswordOverrun { code: i64, message: String },

    /// Password must be modified.
    #[error("password modify required: {message}")]
    UsernamePasswordModify { code: i64, message: String },

    /// Underlying HTTP/transport error.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Invalid/malformed XML in a response.
    #[error("xml parse error: {0}")]
    Xml(#[from] quick_xml::Error),

    /// XML serialization error while building a request body.
    #[error("xml write error: {0}")]
    XmlWrite(#[from] std::io::Error),

    /// JSON parsing error.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// A UTF-8 decoding failure (e.g. when decoding the homepage HTML).
    #[error("utf-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    /// Invalid base64 data.
    #[error("base64 error: {0}")]
    Base64(#[from] base64::DecodeError),

    /// RSA encryption failed.
    #[error("rsa error: {0}")]
    Rsa(String),

    /// Missing/invalid device public key for encryption.
    #[error("no public key was found: {0}")]
    NoPublicKey(String),

    /// The device replied with an unknown/unexpected error code.
    #[error("unknown error: {0}")]
    Unexpected(String),

    /// A generic protocol error (e.g. missing token).
    #[error("protocol error: {0}")]
    Protocol(String),

    /// Any other error.
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Build a `Response` error from a code + optional message.
    pub fn response(code: i64, message: impl Into<String>) -> Self {
        Self::Response {
            code,
            message: message.into(),
        }
    }

    /// Returns the underlying device error code, if any.
    pub fn code(&self) -> Option<i64> {
        use Error::*;
        match self {
            Response { code, .. }
            | NotSupported { code, .. }
            | LoginRequired { code, .. }
            | SystemBusy { code, .. }
            | LoginCsrf { code, .. }
            | WrongSessionToken { code, .. }
            | RequestFormat { code, .. }
            | InvalidCredentials { code, .. }
            | UsernameWrong { code, .. }
            | PasswordWrong { code, .. }
            | AlreadyLogin { code, .. }
            | UsernamePasswordWrong { code, .. }
            | UsernamePasswordOverrun { code, .. }
            | UsernamePasswordModify { code, .. } => Some(*code),
            _ => None,
        }
    }

    /// Whether this error represents a dropped/reset underlying connection,
    /// for which it is sensible to retry the request.
    pub fn is_connection_error(&self) -> bool {
        matches!(self, Error::Http(_))
    }
}
