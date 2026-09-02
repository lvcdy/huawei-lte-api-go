//! VPN related enums.

/// VPN tunnel status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnState {
    /// The VPN is connected.
    Connected = 1,
    /// The VPN is disconnected.
    Disconnected = 0,
}

impl VpnState {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}
