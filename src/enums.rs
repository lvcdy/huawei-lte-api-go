//! Enum types mirroring the Python library's `enums/` package.
//!
//! These are used for strongly-typed method parameters (e.g. which WLAN
//! band to scan, which network mode to set).

pub mod client;
pub mod cradle;
pub mod device;
pub mod dialup;
pub mod net;
pub mod sdcard;
pub mod sms;
pub mod user;
pub mod vpn;
pub mod wlan;
