//! WLAN related enums.

/// The WLAN radio band for scan/list operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlanBand {
    /// 2.4 GHz.
    Band2_4G = 0,
    /// 5 GHz.
    Band5G = 1,
}

impl WlanBand {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

/// The WiFi security/encryption mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlanAuthMode {
    /// WPA2-PSK (AES).
    Wpa2Psk = 3,
    /// Mixed WPA/WPA2-PSK.
    MixedWpaWpa2 = 6,
}

impl WlanAuthMode {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}
