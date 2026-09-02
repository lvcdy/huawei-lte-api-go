//! Cradle (HiLink dock) related enums.

/// The cradle power/battery state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CradleStatus {
    /// The cradle is connected to power.
    Connected = 1,
    /// The cradle is on battery.
    OnBattery = 0,
}

impl CradleStatus {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}
