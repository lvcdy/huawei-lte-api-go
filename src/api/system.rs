//! System API group (`api/System.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::xml::{XmlMap, XmlValue};

use super::{get_value, post_get_value};

/// System API group.
pub struct System<'a> {
    conn: &'a Connection,
}

impl<'a> System<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        System { conn }
    }

    /// `system/devcapacity`. Device capacity.
    pub fn devcapacity(&self) -> Result<Value> {
        get_value(self.conn, "api/system/devcapacity")
    }

    /// `system/deviceinfo`. Device information.
    pub fn deviceinfo(&self) -> Result<Value> {
        get_value(self.conn, "api/system/deviceinfo")
    }

    /// `system/deviceinfoex`. Extended device information.
    pub fn deviceinfoex(&self) -> Result<Value> {
        get_value(self.conn, "api/system/deviceinfoex")
    }

    /// Trigger an online upgrade check.
    pub fn onlineupg(&self) -> Result<Value> {
        let mut inner = XmlMap::new();
        inner.insert("UpdateAction".into(), XmlValue::Text("1".to_string()));
        let mut body = XmlMap::new();
        body.insert("action".into(), XmlValue::Text("check".to_string()));
        body.insert("data".into(), XmlValue::Map(inner));
        post_get_value(self.conn, "api/system/onlineupg", &body)
    }

    /// `system/onlinestate`. Online state of a device.
    pub fn onlinestate(&self, _devid: &str) -> Result<Value> {
        get_value(self.conn, "api/system/onlinestate")
    }

    /// `system/HostInfo`. Host information.
    pub fn hostinfo(&self) -> Result<Value> {
        get_value(self.conn, "api/system/HostInfo")
    }
}
