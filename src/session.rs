//! HTTP session layer that talks to the Huawei CPE.
//!
//! This is the Rust equivalent of the Python library's [`Session`](https://github.com/Salamek/huawei-lte-api/blob/master/huawei_lte_api/Session.py)
//! class: it manages the CSRF token, performs GET/POST requests with the
//! correct headers, parses JSON/XML responses, maps device error codes onto
//! typed [`Error`]s and retries once after a CSRF-invalidated session reloads.
//!
//! API groups do **not** talk to this layer directly; they call it through a
//! [`Connection`](crate::connection::Connection).

use std::cell::{Cell, RefCell};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::errors::{codes, Error, Result};
use crate::xml::{from_xml, json_to_map, to_xml, MapExt, XmlMap, XmlValue};

/// Regex used to extract the CSRF token from the device's homepage HTML.
static CSRF_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"name="csrf_token"\s+content="(\S+)""#).unwrap());

/// Convert a string to CESU-8 bytes, mirroring the Python `cesu8_encode`.
///
/// Characters in the BMP are encoded as normal UTF-8; supplementary (astral)
/// characters are encoded as a CESU-8 surrogate pair (each surrogate itself
/// UTF-8 encoded).
fn cesu8_encode(text: &str) -> Vec<u8> {
    fn push_utf8(code: u32, out: &mut Vec<u8>) {
        if code < 0x80 {
            out.push(code as u8);
        } else if code < 0x800 {
            out.push(0xC0 | (code >> 6) as u8);
            out.push(0x80 | (code & 0x3F) as u8);
        } else if code < 0x10000 {
            out.push(0xE0 | (code >> 12) as u8);
            out.push(0x80 | ((code >> 6) & 0x3F) as u8);
            out.push(0x80 | (code & 0x3F) as u8);
        } else {
            out.push(0xF0 | (code >> 18) as u8);
            out.push(0x80 | ((code >> 12) & 0x3F) as u8);
            out.push(0x80 | ((code >> 6) & 0x3F) as u8);
            out.push(0x80 | (code & 0x3F) as u8);
        }
    }

    let mut out = Vec::with_capacity(text.len());
    for ch in text.chars() {
        let code = ch as u32;
        if code <= 0xFFFF {
            push_utf8(code, &mut out);
        } else {
            let base = code - 0x10000;
            push_utf8(0xD800 + (base >> 10), &mut out);
            push_utf8(0xDC00 + (base & 0x3FF), &mut out);
        }
    }
    out
}

/// Whether `blob[i..]` starts with a CESU-8 encoded high surrogate.
///
/// A high surrogate (U+D800..U+DBFF) is UTF-8 encoded as
/// `0xED 0xA0-0xAF 0x80-0xBF`.
fn is_high_surrogate(blob: &[u8], i: usize) -> bool {
    i + 3 <= blob.len()
        && blob[i] == 0xED
        && (0xA0..=0xAF).contains(&blob[i + 1])
        && (0x80..=0xBF).contains(&blob[i + 2])
}

/// Whether `blob[i..]` starts with a CESU-8 encoded low surrogate.
///
/// A low surrogate (U+DC00..U+DFFF) is UTF-8 encoded as
/// `0xED 0xB0-0xBF 0x80-0xBF`.
fn is_low_surrogate(blob: &[u8], i: usize) -> bool {
    i + 3 <= blob.len()
        && blob[i] == 0xED
        && (0xB0..=0xBF).contains(&blob[i + 1])
        && (0x80..=0xBF).contains(&blob[i + 2])
}

/// Decode a CESU-8 encoded high/low surrogate pair back into the original
/// supplementary code point, returned as a 4-byte UTF-8 sequence.
fn decode_surrogate_pair(blob: &[u8], i: usize) -> u32 {
    // Each surrogate is a 3-byte UTF-8 sequence for a code point in
    // U+D800..U+DFFF: ED <6-bit payload> <6-bit payload>.
    let hi = (((blob[i] & 0x0F) as u32) << 12)
        | (((blob[i + 1] & 0x3F) as u32) << 6)
        | ((blob[i + 2] & 0x3F) as u32);
    let lo = (((blob[i + 3] & 0x0F) as u32) << 12)
        | (((blob[i + 4] & 0x3F) as u32) << 6)
        | ((blob[i + 5] & 0x3F) as u32);
    0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00)
}

/// Decode CESU-8 encoded bytes back to standard UTF-8, mirroring the Python
/// `cesu8_fix`. Surrogate pairs produced by `cesu8_encode` are collapsed back
/// into the original supplementary character.
fn cesu8_fix(blob: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(blob.len());
    let mut i = 0;
    while i < blob.len() {
        if is_high_surrogate(blob, i) && is_low_surrogate(blob, i + 3) {
            let code = decode_surrogate_pair(blob, i);
            // encode `code` as 4-byte UTF-8
            out.push(0xF0 | (code >> 18) as u8);
            out.push(0x80 | ((code >> 12) & 0x3F) as u8);
            out.push(0x80 | ((code >> 6) & 0x3F) as u8);
            out.push(0x80 | (code & 0x3F) as u8);
            i += 6;
        } else {
            out.push(blob[i]);
            i += 1;
        }
    }
    out
}

/// A low-level HTTP exchange: the response headers and raw body bytes.
///
/// This is the unit of work performed by [`Session`] against the network.
/// The default [`reqwest`] transport is used unless a custom
/// [`HttpTransport`] is injected (see [`Session::with_transport`]).
pub type HttpExchange = (reqwest::header::HeaderMap, Vec<u8>);

/// Abstraction over the HTTP layer so tests can inject canned responses.
///
/// A [`Session`] normally talks to the device through `reqwest`. By providing
/// a custom transport (e.g. an in-memory fake in tests) the whole request →
/// parse pipeline — headers, CSRF tokens, JSON/XML decoding — can be exercised
/// without a real device.
pub trait HttpTransport: Send + Sync {
    /// Perform a GET and return the response headers and raw body bytes.
    fn get(&self, url: &str, token: Option<&str>) -> Result<HttpExchange>;

    /// Perform a POST and return the response headers and raw body bytes.
    fn post(&self, url: &str, token: Option<&str>, body: &[u8]) -> Result<HttpExchange>;
}

/// A live, stateful HTTP session to the device.
///
/// Methods take `&self` (interior mutability) so a single session can be
/// shared through a shared reference, matching the `Connection::session()`
/// accessor that returns `&Session`.
pub struct Session {
    /// The base URL (always ends with `/`).
    url: String,
    /// The underlying HTTP client (cookies enabled).
    client: reqwest::blocking::Client,
    /// Optional injected transport (used by tests instead of `client`).
    transport: Option<Box<dyn HttpTransport>>,
    /// The current CSRF tokens; the last one is sent on requests.
    csrf_tokens: RefCell<Vec<String>>,
    /// Whether the session is marked authenticated (set after a successful
    /// login; not otherwise consulted by the request path, which keys off the
    /// presence of CSRF tokens, mirroring the Python behaviour).
    authenticated: Cell<bool>,
    /// Cached `webserver/publickey` result.
    encryption_key: RefCell<Option<XmlMap>>,
    /// Cached RSA padding type from `user/state-login`.
    rsa_padding: Cell<u8>,
}

impl Session {
    /// Create a new session for `base_url` and initialize its CSRF token.
    pub fn new(base_url: &str) -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()?;

        // Strip embedded credential info, like the Python Session does.
        let mut url = base_url.to_string();
        if let Some(at) = url.rfind('@') {
            // Only strip when it looks like `scheme://user:pass@host`.
            if url[..at].contains("://") && !url[..at].contains('/') {
                url = url[at + 1..].to_string();
            }
        }
        if !url.ends_with('/') {
            url.push('/');
        }

        let session = Session {
            url,
            client,
            transport: None,
            csrf_tokens: RefCell::new(Vec::new()),
            authenticated: Cell::new(false),
            encryption_key: RefCell::new(None),
            rsa_padding: Cell::new(0),
        };
        session.reload()?;
        Ok(session)
    }

    /// Create a new session backed by a custom [`HttpTransport`].
    ///
    /// Useful for tests: the injected transport receives every request and can
    /// return canned responses. The session is still initialised by fetching
    /// the CSRF token from the (fake) homepage, so the transport must handle
    /// the initial GET to the base URL.
    pub fn with_transport(base_url: &str, transport: Box<dyn HttpTransport>) -> Result<Self> {
        // Strip embedded credential info, like the Python Session does.
        let mut url = base_url.to_string();
        if let Some(at) = url.rfind('@') {
            if url[..at].contains("://") && !url[..at].contains('/') {
                url = url[at + 1..].to_string();
            }
        }
        if !url.ends_with('/') {
            url.push('/');
        }

        let session = Session {
            url,
            client: reqwest::blocking::Client::new(),
            transport: Some(transport),
            csrf_tokens: RefCell::new(Vec::new()),
            authenticated: Cell::new(false),
            encryption_key: RefCell::new(None),
            rsa_padding: Cell::new(0),
        };
        session.reload()?;
        Ok(session)
    }

    /// The base URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// The isolation HTTP client (exposed for advanced callers).
    pub fn client(&self) -> &reqwest::blocking::Client {
        &self.client
    }

    /// The first CSRF token, used to derive the SHA256 login password.
    ///
    /// Mirrors Python's `request_verification_tokens[0]`.
    pub fn csrf_token(&self) -> Option<String> {
        self.csrf_tokens.borrow().first().cloned()
    }

    /// All current CSRF tokens (last one is sent on requests).
    pub fn csrf_tokens(&self) -> Vec<String> {
        self.csrf_tokens.borrow().clone()
    }

    /// Re-fetch the CSRF token from the homepage (falling back to the API).
    pub fn reload(&self) -> Result<()> {
        *self.csrf_tokens.borrow_mut() = Vec::new();

        let body = if let Some(t) = &self.transport {
            t.get(&self.url, None)?.1
        } else {
            self.client.get(&self.url).send()?.bytes()?.to_vec()
        };
        let text = String::from_utf8_lossy(&body).to_string();

        let tokens: Vec<String> = CSRF_RE
            .captures_iter(&text)
            .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
            .collect();

        if !tokens.is_empty() {
            *self.csrf_tokens.borrow_mut() = tokens;
            return Ok(());
        }

        if let Some(token) = self.get_token()? {
            self.csrf_tokens.borrow_mut().push(token);
        }
        Ok(())
    }

    /// Whether the session has been marked authenticated.
    pub fn is_authenticated(&self) -> bool {
        self.authenticated.get()
    }

    /// Mark the session authenticated (called after a successful login).
    pub fn set_authenticated(&self) {
        self.authenticated.set(true);
    }

    /// Clear the authenticated flag (called after logout).
    pub fn clear_authenticated(&self) {
        self.authenticated.set(false);
    }

    /// Fetch `api/webserver/SesTokInfo`, which provides the CSRF tokens and
    /// the RSA public-key material used for login. Returns the full parsed
    /// response (with a `response` root).
    pub fn request_session_tokens(&self) -> Result<XmlMap> {
        self.get_xml("api/webserver/SesTokInfo")
    }

    /// Need the encryption key. Mirrors the Python `_get_encryption_key`.
    fn encryption_key(&self) -> XmlMap {
        let mut cached = self.encryption_key.borrow_mut();
        if cached.is_none() {
            *cached = self.get_xml("api/webserver/publickey").ok();
        }
        cached.clone().unwrap_or_default()
    }

    /// Need the RSA padding type. Mirrors the Python `_get_rsa_padding`.
    fn rsa_padding_type(&self) -> u8 {
        let padding = self.rsa_padding.get();
        if padding != 0 {
            return padding;
        }
        let padding = self
            .get_xml("api/user/state-login")
            .ok()
            .and_then(|d| d.get_str(&["response", "rsapadingtype"]))
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(0);
        self.rsa_padding.set(if padding == 0 { 0 } else { padding });
        padding
    }

    /// The RSA padding type (pow used by external callers that encrypt bodies).
    pub fn rsa_padding(&self) -> u8 {
        self.rsa_padding_type()
    }

    /// The public key `(e, n)` as hex strings, if the device provides one.
    pub fn public_key(&self) -> (String, String) {
        let key = self.encryption_key();
        let e = key.get_str(&["response", "encpubkeye"]).unwrap_or_default();
        let n = key.get_str(&["response", "encpubkeyn"]).unwrap_or_default();
        (e, n)
    }

    fn build_final_url(&self, endpoint: &str) -> String {
        format!("{}{}", self.url, endpoint)
    }

    /// Issue a GET request to `endpoint` (a full path such as
    /// `"api/device/information"`), parsing and status-checking the response.
    pub fn get_xml(&self, endpoint: &str) -> Result<XmlMap> {
        match self.get_once(endpoint) {
            Err(Error::LoginCsrf { .. }) => {
                self.reload()?;
                self.get_once(endpoint)
            }
            result => result,
        }
    }

    fn get_once(&self, endpoint: &str) -> Result<XmlMap> {
        let tokens = self.csrf_tokens.borrow();
        let token = if tokens.len() == 1 {
            Some(tokens[0].clone())
        } else {
            None
        };
        drop(tokens);

        let (headers, bytes) = if let Some(t) = &self.transport {
            t.get(&self.build_final_url(endpoint), token.as_deref())?
        } else {
            let mut req = self.client.get(self.build_final_url(endpoint));
            if let Some(token) = &token {
                req = req.header("__RequestVerificationToken", token);
            }
            let resp = req.send()?;
            let headers = resp.headers().clone();
            let bytes = resp.bytes()?.to_vec();
            (headers, bytes)
        };
        self.process_response(&headers, &bytes)
    }

    /// Issue a POST request with an XML body, parsing/status-checking the
    /// response and storing any new CSRF tokens from the response headers.
    pub fn post_xml(&self, endpoint: &str, data: &XmlMap) -> Result<XmlMap> {
        match self.post_once(endpoint, data, false) {
            Err(Error::LoginCsrf { .. }) => {
                self.reload()?;
                self.post_once(endpoint, data, false)
            }
            result => result,
        }
    }

    /// Issue a POST request with an empty body (used for e.g. logout).
    pub fn post_empty_xml(&self, endpoint: &str) -> Result<XmlMap> {
        let empty = XmlMap::new();
        self.post_xml(endpoint, &empty)
    }

    /// Issue a POST request and force a CSRF refresh afterwards
    /// (`refresh_csrf` in the Python library).
    pub fn post_xml_refresh(&self, endpoint: &str, data: &XmlMap) -> Result<XmlMap> {
        match self.post_once(endpoint, data, true) {
            Err(Error::LoginCsrf { .. }) => {
                self.reload()?;
                self.post_once(endpoint, data, true)
            }
            result => result,
        }
    }

    /// The one-shot POST (no retry). Mirrors the Python `_post`.
    fn post_once(&self, endpoint: &str, data: &XmlMap, refresh_csrf: bool) -> Result<XmlMap> {
        let body = cesu8_encode(&to_xml(data));

        // Grab the token to send (consuming the first of several, like Python).
        let token_to_send = {
            let mut tokens = self.csrf_tokens.borrow_mut();
            if tokens.is_empty() {
                None
            } else if tokens.len() > 1 {
                Some(tokens.remove(0))
            } else {
                Some(tokens[0].clone())
            }
        };

        let (headers, bytes) = if let Some(t) = &self.transport {
            if refresh_csrf {
                *self.csrf_tokens.borrow_mut() = Vec::new();
            }
            let (headers, bytes) = t.post(
                &self.build_final_url(endpoint),
                token_to_send.as_deref(),
                &body,
            )?;
            self.ingest_csrf_headers(&headers);
            (headers, bytes)
        } else {
            let mut req = self
                .client
                .post(self.build_final_url(endpoint))
                .header("Content-Type", "application/xml");
            if let Some(token) = &token_to_send {
                req = req.header("__RequestVerificationToken", token);
            }
            let resp = req.body(body).send()?;
            if refresh_csrf {
                *self.csrf_tokens.borrow_mut() = Vec::new();
            }
            let headers = resp.headers().clone();
            // Extract fresh CSRF tokens from the response headers.
            self.ingest_csrf_headers(&headers);
            let bytes = resp.bytes()?.to_vec();
            (headers, bytes)
        };
        self.process_response(&headers, &bytes)
    }

    /// Extract fresh CSRF tokens from the response headers.
    fn ingest_csrf_headers(&self, headers: &reqwest::header::HeaderMap) {
        if let Some(v) = headers.get("__RequestVerificationTokenone") {
            if let Ok(s) = v.to_str() {
                let mut t = self.csrf_tokens.borrow_mut();
                t.push(s.to_string());
                if let Some(v2) = headers.get("__RequestVerificationTokentwo") {
                    if let Ok(s2) = v2.to_str() {
                        t.push(s2.to_string());
                    }
                }
            }
        } else if let Some(v) = headers.get("__RequestVerificationToken") {
            if let Ok(s) = v.to_str() {
                self.csrf_tokens.borrow_mut().push(s.to_string());
            }
        }
    }

    /// Parse the response body (JSON or XML), translating device error codes
    /// into typed [`Error`]s.
    fn process_response(
        &self,
        headers: &reqwest::header::HeaderMap,
        bytes: &[u8],
    ) -> Result<XmlMap> {
        let is_json = detect_json(bytes, headers);
        let map: XmlMap = if is_json {
            let value: serde_json::Value = if bytes.is_empty() {
                serde_json::Value::Null
            } else {
                serde_json::from_slice(bytes)?
            };
            match json_to_map(&value) {
                XmlValue::Map(m) => m,
                other => {
                    let mut m = XmlMap::new();
                    m.insert("response".into(), other);
                    m
                }
            }
        } else if bytes.is_empty() {
            XmlMap::new()
        } else {
            from_xml(&cesu8_fix(bytes))?
        };

        check_response_status(&map)
    }

    /// Fetch a CSRF token from the API when it isn't in the homepage HTML.
    ///
    /// Uses the non-retrying `get_once` so a CSRF failure here cannot recurse
    /// back into `reload`.
    fn get_token(&self) -> Result<Option<String>> {
        match self.get_once("api/webserver/token") {
            Ok(data) => {
                let token = data
                    .get_map(&["response", "token"])
                    .and_then(|m| m.get("token").map(XmlValue::as_str))
                    .or_else(|| data.get_str(&["token"]));
                Ok(token)
            }
            Err(Error::NotSupported { .. }) => match self.get_once("api/webserver/SesTokInfo") {
                Ok(data) => Ok(data.get_str(&["response", "TokInfo"])),
                Err(Error::NotSupported { .. }) => Ok(None),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

/// Decide whether the body is JSON based on Content-Type first, then a
/// content sniff of the first byte.
fn detect_json(bytes: &[u8], headers: &reqwest::header::HeaderMap) -> bool {
    if let Some(ct) = headers.get(reqwest::header::CONTENT_TYPE) {
        if let Ok(ct) = ct.to_str() {
            let ctype = ct
                .split(';')
                .next()
                .unwrap_or(ct)
                .trim()
                .to_ascii_lowercase();
            if ctype.ends_with("/json") || ctype.ends_with("+json") {
                return true;
            }
            if ctype.ends_with("/xml") || ctype.ends_with("+xml") {
                return false;
            }
        }
    }
    !bytes.is_empty() && (bytes[0] == b'{' || bytes[0] == b'[')
}

/// Translate the parsed response into a typed result, mirroring the Python
/// `_check_response_status`.
fn check_response_status(map: &XmlMap) -> Result<XmlMap> {
    if let Some(XmlValue::Map(err)) = map.get("error") {
        let code = err
            .get_str(&["code"])
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0);
        let mut message = err.get_str(&["message"]).unwrap_or_default();
        if message.is_empty() {
            message = error_code_to_message(code).to_string();
        }
        return Err(make_error(code, message));
    }
    Ok(map.clone())
}

fn error_code_to_message(code: i64) -> &'static str {
    match code {
        codes::ERROR_SYSTEM_BUSY => "System busy",
        codes::ERROR_SYSTEM_NO_RIGHTS => "No rights (needs login)",
        codes::ERROR_SYSTEM_NO_SUPPORT => "No support",
        codes::ERROR_SYSTEM_UNKNOWN => "Unknown",
        codes::ERROR_SYSTEM_CSRF => "Session error",
        codes::ERROR_WRONG_SESSION_TOKEN => "Wrong Session Token",
        codes::ERROR_FORMAT_ERROR => "Request format error",
        _ => "Unknown",
    }
}

/// Map a device error code to the corresponding typed [`Error`] variant.
fn make_error(code: i64, message: String) -> Error {
    const LOGIN_USERNAME_WRONG: i64 = 108001;
    const LOGIN_PASSWORD_WRONG: i64 = 108002;
    const LOGIN_ALREADY_LOGIN: i64 = 108003;
    const LOGIN_USERNAME_PASSWORD_WRONG: i64 = 108006;
    const LOGIN_USERNAME_PASSWORD_OVERRUN: i64 = 108007;
    const LOGIN_PASSWORD_MODIFY: i64 = 115002;

    match code {
        codes::ERROR_SYSTEM_NO_SUPPORT => Error::NotSupported { code, message },
        codes::ERROR_SYSTEM_NO_RIGHTS => Error::LoginRequired { code, message },
        codes::ERROR_SYSTEM_BUSY => Error::SystemBusy { code, message },
        codes::ERROR_SYSTEM_CSRF => Error::LoginCsrf { code, message },
        codes::ERROR_WRONG_SESSION_TOKEN => Error::WrongSessionToken { code, message },
        codes::ERROR_FORMAT_ERROR => Error::RequestFormat { code, message },
        LOGIN_USERNAME_WRONG => Error::UsernameWrong { code, message },
        LOGIN_PASSWORD_WRONG => Error::PasswordWrong { code, message },
        LOGIN_ALREADY_LOGIN => Error::AlreadyLogin { code, message },
        LOGIN_USERNAME_PASSWORD_WRONG => Error::UsernamePasswordWrong { code, message },
        LOGIN_USERNAME_PASSWORD_OVERRUN => Error::UsernamePasswordOverrun { code, message },
        LOGIN_PASSWORD_MODIFY => Error::UsernamePasswordModify { code, message },
        _ => Error::Response { code, message },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cesu8_roundtrip_bmp() {
        let encoded = cesu8_encode("abc");
        assert_eq!(encoded, b"abc");
    }

    #[test]
    fn cesu8_astral_is_reversible() {
        // U+1F600 (😀) is astral.
        let text = "\u{1F600}";
        let encoded = cesu8_encode(text);
        assert!(
            encoded.len() >= 6,
            "expected surrogate pair, got {encoded:?}"
        );
        let fixed = cesu8_fix(&encoded);
        assert_eq!(String::from_utf8(fixed).unwrap(), text);
    }

    #[test]
    fn cesu8_fix_preserves_normal_utf8_chinese() {
        // Real device: api/net/current-plmn returns "中国移动" as plain UTF-8
        // bytes (e4 b8 ad e5 9b bd e7 a7 bb e5 8a a8). cesu8_fix must NOT
        // reinterpret these as CESU-8 surrogate pairs.
        let cn = "中国移动";
        let bytes = cn.as_bytes();
        let fixed = cesu8_fix(bytes);
        assert_eq!(String::from_utf8(fixed).unwrap(), cn);
    }

    #[test]
    fn detect_json_content_type() {
        let mut h = reqwest::header::HeaderMap::new();
        h.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        assert!(detect_json(b"{}", &h));
        // JSON content type takes precedence over content sniffing.
        assert!(detect_json(b"{", &h));

        // XML content type takes precedence over the leading brace.
        let mut xh = reqwest::header::HeaderMap::new();
        xh.insert(reqwest::header::CONTENT_TYPE, "text/xml".parse().unwrap());
        assert!(!detect_json(b"{", &xh));
    }
}
