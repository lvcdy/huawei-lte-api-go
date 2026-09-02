//! App API group (`api/App.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::xml::{XmlMap, XmlValue};

use super::{get_value, post_get_value};

/// App API group.
pub struct App<'a> {
    conn: &'a Connection,
}

impl<'a> App<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        App { conn }
    }

    /// `app/operatorinfo`. Operator information.
    pub fn operatorinfo(&self, _lang: &str) -> Result<Value> {
        get_value(self.conn, "api/app/operatorinfo")
    }

    /// `app/privacypolicy`. Privacy policy.
    pub fn privacypolicy(&self, _lang: &str) -> Result<Value> {
        get_value(self.conn, "api/app/privacypolicy")
    }

    /// Accept or decline the privacy policy.
    ///
    /// * `approve` — `true` to approve, `false` to decline.
    pub fn accept_privacypolicy(&self, approve: bool) -> Result<Value> {
        let mut inner = XmlMap::new();
        inner.insert(
            "Approve".into(),
            XmlValue::Text(if approve {
                "2".to_string()
            } else {
                "0".to_string()
            }),
        );
        inner.insert("Liscence".into(), XmlValue::Text("0".to_string()));
        let mut body = XmlMap::new();
        body.insert("data".into(), XmlValue::Map(inner));
        post_get_value(self.conn, "api/app/privacypolicy", &body)
    }
}
