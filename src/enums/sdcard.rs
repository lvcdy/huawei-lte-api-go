//! SD-card related enums.

/// The SD-card share mode (whether the device exposes it over SMB/etc).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdCardShareMode {
    /// The SD card is not shared.
    NotShared = 0,
    /// The SD card is shared.
    Shared = 1,
}

impl SdCardShareMode {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}
