//! Dial-up (mobile data connection) related enums.

/// Whether the mobile data connection should be enabled/disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialUpAction {
    /// Disconnect the data connection.
    Disconnect = 0,
    /// Connect the data connection.
    Connect = 1,
}

impl DialUpAction {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}
