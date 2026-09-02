//! Network related enums, mirroring `enums/net.py`.

use std::fmt::Write as _;

/// Network mode to apply to the device (`NetworkModeEnum`).
///
/// Values are two-digit strings as sent in the `NetworkMode` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkMode {
    /// Auto.
    ModeAuto,
    /// 2G only.
    Mode2GOnly,
    /// 3G only.
    Mode3GOnly,
    /// 3G/2G auto.
    Mode3G2GAuto,
    /// 4G only.
    Mode4GOnly,
    /// 4G/2G auto.
    Mode4G2GAuto,
    /// 4G/3G auto.
    Mode4G3GAuto,
}

impl NetworkMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            NetworkMode::ModeAuto => "00",
            NetworkMode::Mode2GOnly => "01",
            NetworkMode::Mode3GOnly => "02",
            NetworkMode::Mode3G2GAuto => "0201",
            NetworkMode::Mode4GOnly => "03",
            NetworkMode::Mode4G2GAuto => "0301",
            NetworkMode::Mode4G3GAuto => "0302",
        }
    }
}

/// 3G network band bitmask (`NetworkBandEnum`). Combine with `|`; use `All`
/// for all bands / when not applicable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NetworkBand {
    Bc0a = 0x01,
    Bc0b = 0x02,
    Bc1 = 0x04,
    Bc2 = 0x08,
    Bc3 = 0x10,
    Bc4 = 0x20,
    Bc5 = 0x40,
    Gsm1800 = 0x80,
    Gsm900 = 0x300,
    Bc6 = 0x400,
    Bc7 = 0x800,
    Bc8 = 0x1000,
    Bc9 = 0x2000,
    Bc10 = 0x4000,
    Bc11 = 0x8000,
    Gsm850 = 0x80000,
    Gsm1900 = 0x200000,
    UmtsB1_2100 = 0x400000,
    UmtsB2_1900 = 0x800000,
    Bc12 = 0x10000000,
    Bc13 = 0x20000000,
    UmtsB5_850 = 0x4000000,
    Bc14 = 0x80000000,
    UmtsB8_900 = 0x2000000000000,
    /// All bands — use alone, do not `|` with others.
    All = 0x3FFFFFFF,
}

impl NetworkBand {
    pub fn as_u64(&self) -> u64 {
        *self as u64
    }
}

/// LTE band bitmask (`LTEBandEnum`). Combine with `|`; use `All` for all
/// bands / when not applicable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum LTEBand {
    B1 = 0x01,
    B3 = 0x04,
    B7 = 0x40,
    B8 = 0x80,
    B20 = 0x80000,
    B28 = 0x8000000,
    B38 = 0x2000000000,
    B40 = 0x8000000000,
    /// All bands — use alone, do not `|` with others.
    All = 0x7FFFFFFFFFFFFFFF,
}

impl LTEBand {
    pub fn as_u64(&self) -> u64 {
        *self as u64
    }
}

/// Format a band bitmask as the hex string expected by `LTEBand`/`NetworkBand`
/// device fields.
pub fn band_to_hex(band: u64) -> String {
    let mut s = String::new();
    let _ = write!(s, "{:x}", band);
    s
}

/// PLMN (public land mobile network) scan mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlmnMode {
    /// Automatic network selection.
    Auto = 0,
    /// Manual network selection.
    Manual = 1,
}

impl PlmnMode {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}
