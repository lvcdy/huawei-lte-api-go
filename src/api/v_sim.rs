//! VSim API group (`api/VSim.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;

use super::get_value;

/// VSim API group.
pub struct VSim<'a> {
    conn: &'a Connection,
}

impl<'a> VSim<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        VSim { conn }
    }

    /// `vsim/operateswitch-vsim`. Operate the VSim switch.
    pub fn operateswitch_vsim(&self) -> Result<Value> {
        get_value(self.conn, "api/vsim/operateswitch-vsim")
    }
}
