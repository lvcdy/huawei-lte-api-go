//! `config/*.py` groups. Each mirroring one of the Python library's
//! `config/*.py` classes. Endpoints are requested via the `config` prefix.

#[cfg(test)]
mod config_test;
pub mod device;
pub mod device_information;
pub mod dial_up;
pub mod fast_boot;
pub mod firewall;
pub mod global;
pub mod ipv6;
pub mod lan;
pub mod network;
pub mod ota;
pub mod pb;
pub mod pc_assistant;
pub mod pincode;
pub mod sms;
pub mod sntp;
pub mod statistic;
pub mod stk;
pub mod u_pnp;
pub mod update;
pub mod ussd;
pub mod voice;
pub mod web_sd;
pub mod web_ui_cfg;
pub mod wifi;
