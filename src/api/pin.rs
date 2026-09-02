//! Pin API group (`api/Pin.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_set};

/// Pin API group.
pub struct Pin<'a> {
    conn: &'a Connection,
}

impl<'a> Pin<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Pin { conn }
    }

    /// `pin/status`. PIN status.
    pub fn status(&self) -> Result<Value> {
        get_value(self.conn, "api/pin/status")
    }

    /// `pin/simlock`. SIM lock status.
    pub fn simlock(&self) -> Result<Value> {
        get_value(self.conn, "api/pin/simlock")
    }

    /// `pin/save-pin`. Save PIN.
    pub fn save_pin(&self) -> Result<Value> {
        get_value(self.conn, "api/pin/save-pin")
    }

    /// Perform an operation on the PIN.
    ///
    /// * `operate_type` — `0` verify, `1` enable, `2` disable, `3` set new,
    ///   `4` use PUK.
    /// * `current_pin` / `new_pin` / `puk_code` — optional, empty when unused.
    pub fn operate(
        &self,
        operate_type: &str,
        current_pin: Option<&str>,
        new_pin: Option<&str>,
        puk_code: Option<&str>,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            ("OperateType", operate_type.to_string()),
            ("CurrentPin", current_pin.unwrap_or("").to_string()),
            ("NewPin", new_pin.unwrap_or("").to_string()),
            ("PukCode", puk_code.unwrap_or("").to_string()),
        ]);
        post_set(self.conn, "api/pin/operate", &body)
    }
}
