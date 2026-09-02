//! Device related enums.

/// The device boot mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootMode {
    /// Normal boot.
    Normal = 0,
    /// Fast boot.
    FastBoot = 1,
}

impl BootMode {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

/// The antenna configuration (`AntennaTypeEnum`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntennaType {
    /// Internal antennas.
    Integrated = 0,
    /// Two external antennas.
    External1And2 = 1,
    /// One external antenna.
    External1 = 2,
    /// Automatic selection.
    Auto = 3,
}

impl AntennaType {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

/// The device control action (`ControlModeEnum`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlMode {
    /// Reboot the device.
    Reboot = 1,
    /// Reset to factory settings.
    Reset = 2,
    /// Back up the configuration (downloadable from the web UI).
    BackupConfiguration = 3,
    /// Power off.
    PowerOff = 4,
}

impl ControlMode {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

/// The device mode (`ModeEnum`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Normal (production) mode.
    Normal = 0,
    /// Debug mode.
    Debug = 1,
    /// Enable telnet.
    EnableTelnet = 2,
}

impl Mode {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}
