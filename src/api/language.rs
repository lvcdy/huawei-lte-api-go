//! Language API group (`api/Language.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_set};

/// Language API group.
pub struct Language<'a> {
    conn: &'a Connection,
}

impl<'a> Language<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Language { conn }
    }

    /// Set the current language.
    ///
    /// * `current_language` — language code (e.g. `"en_us"`).
    pub fn set_current_language(&self, current_language: &str) -> Result<String> {
        let body: XmlMap = map_of([("CurrentLanguage", current_language.to_string())]);
        post_set(self.conn, "api/language/current-language", &body)
    }

    /// `language/current-language`. Current language.
    pub fn current_language(&self) -> Result<Value> {
        get_value(self.conn, "api/language/current-language")
    }
}
