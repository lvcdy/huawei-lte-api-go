//! Test support: an in-memory fake [`HttpTransport`] plus helpers to build a
//! [`Connection`] backed by it.
//!
//! This module is only compiled for tests (see the `#[cfg(test)]` gate in
//! `lib.rs`). It lets API-group tests exercise the full request → parse
//! pipeline — headers, CSRF tokens, JSON/XML decoding, error mapping —
//! without a real device.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use reqwest::header::HeaderMap;

use crate::connection::Connection;
use crate::session::{HttpExchange, HttpTransport};
use crate::Result;

/// A canned response: the headers and raw body the fake transport returns.
#[derive(Clone)]
pub struct FakeResponse {
    /// Response headers (e.g. `Content-Type`, `__RequestVerificationToken`).
    pub headers: HeaderMap,
    /// Raw body bytes.
    pub body: Vec<u8>,
}

impl FakeResponse {
    /// Build a response from a body with the given content type.
    pub fn new(content_type: &str, body: impl Into<Vec<u8>>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            content_type.parse().expect("valid content type"),
        );
        FakeResponse {
            headers,
            body: body.into(),
        }
    }

    /// Build a response from an XML string body.
    pub fn xml(body: impl Into<String>) -> Self {
        Self::new("text/xml", body.into().into_bytes())
    }

    /// Build a response from a JSON string body.
    pub fn json(body: impl Into<String>) -> Self {
        Self::new("application/json", body.into().into_bytes())
    }

    /// Convenience: a `text/xml` `<response><OK>OK</OK></response>` body.
    pub fn ok() -> Self {
        Self::xml("<response><OK>OK</OK></response>")
    }
}

/// A single recorded request made through the fake transport.
#[derive(Debug, Clone)]
pub struct RecordedRequest {
    /// Full URL that was requested.
    pub url: String,
    /// The CSRF token sent with the request, if any.
    pub token: Option<String>,
    /// The raw request body (empty for GETs).
    pub body: Vec<u8>,
}

/// The shared mutable state of a [`FakeTransport`].
#[derive(Default)]
struct FakeState {
    responses: HashMap<String, FakeResponse>,
    requests: Vec<RecordedRequest>,
}

/// An in-memory fake HTTP transport.
///
/// Routes are matched by URL substring. Requests are recorded (for asserting
/// on URLs, CSRF tokens and POST bodies), and the stored responses are
/// returned. The transport is cheaply cloneable (`Arc`-backed), so a test can
/// hand one to a [`Connection`] and still inspect the recorded traffic.
#[derive(Clone, Default)]
pub struct FakeTransport {
    state: Arc<Mutex<FakeState>>,
}

impl FakeTransport {
    /// Create an empty fake transport.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register the response returned for requests whose URL contains `path`.
    pub fn route(&self, path: &str, response: FakeResponse) -> &Self {
        self.state
            .lock()
            .unwrap()
            .responses
            .insert(path.to_string(), response);
        self
    }

    /// Register the XML response returned for requests whose URL contains
    /// `path` (convenience wrapper around [`FakeTransport::route`]).
    pub fn route_xml(&self, path: &str, body: impl Into<String>) -> &Self {
        self.route(path, FakeResponse::xml(body))
    }

    /// The requests received so far (in order).
    pub fn requests(&self) -> Vec<RecordedRequest> {
        self.state.lock().unwrap().requests.clone()
    }

    /// The body of the request matching `path` (first match), if any.
    pub fn body_for(&self, path: &str) -> Option<Vec<u8>> {
        self.requests()
            .into_iter()
            .find(|r| r.url.contains(path))
            .map(|r| r.body)
    }

    /// The body of the request matching `path` as a lossy string.
    pub fn body_string_for(&self, path: &str) -> String {
        self.body_for(path)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default()
    }

    /// The CSRF token used on the request matching `path` (first match).
    pub fn token_for(&self, path: &str) -> Option<String> {
        self.requests()
            .into_iter()
            .find(|r| r.url.contains(path))
            .and_then(|r| r.token)
    }

    fn record(&self, url: &str, token: Option<&str>, body: &[u8]) {
        self.state.lock().unwrap().requests.push(RecordedRequest {
            url: url.to_string(),
            token: token.map(|s| s.to_string()),
            body: body.to_vec(),
        });
    }

    fn respond(&self, url: &str) -> (HeaderMap, Vec<u8>) {
        // Match the longest registered path that occurs in `url`. This keeps a
        // catch-all route like "/" from shadowing more specific routes.
        let resp = self
            .state
            .lock()
            .unwrap()
            .responses
            .iter()
            .filter(|(path, _)| url.contains(path.as_str()))
            .max_by_key(|(path, _)| path.len())
            .map(|(_, r)| r.clone())
            .unwrap_or_else(FakeResponse::ok);
        (resp.headers, resp.body)
    }
}

impl HttpTransport for FakeTransport {
    fn get(&self, url: &str, token: Option<&str>) -> Result<HttpExchange> {
        self.record(url, token, &[]);
        Ok(self.respond(url))
    }

    fn post(&self, url: &str, token: Option<&str>, body: &[u8]) -> Result<HttpExchange> {
        self.record(url, token, body);
        Ok(self.respond(url))
    }
}

/// Build a [`Connection`] backed by a [`FakeTransport`].
///
/// A homepage response embedding a CSRF token is registered automatically so
/// the session can initialise (the transport's constructor fetches the base
/// URL to discover CSRF tokens).
pub fn conn_with() -> (Connection, FakeTransport) {
    let transport = FakeTransport::new();
    transport.route(
        "/",
        FakeResponse::new(
            "text/html",
            br#"<html><head><meta name="csrf_token" content="CSRF_TOKEN_HOME"></head></html>"#
                .to_vec(),
        ),
    );
    let conn = Connection::with_transport("http://cpe.local/", Box::new(transport.clone()))
        .expect("connection with fake transport");
    (conn, transport)
}
