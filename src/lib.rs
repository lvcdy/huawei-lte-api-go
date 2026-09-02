//! # huawei-lte-api
//!
//! A Rust port of the [`huawei-lte-api`](https://github.com/Salamek/huawei-lte-api)
//! Python library for interacting with Huawei LTE/5G CPE devices (mobile routers).
//!
//! It re-implements the protocol from scratch in an idiomatic Rust way (it is **not**
//! a line-by-line translation of the Python code), and additionally includes
//! supplementary endpoints found in the
//! [`Brovi-Huawei-5G-CPE-Manager`](https://github.com/fz911a/Brovi-Huawei-5G-CPE-Manager)
//! Android app (5G NR cell info, band/cell locking, developer mode, etc.).
//!
//! ## Quick start
//!
//! ```no_run
//! use huawei_lte_api::{Client, Connection};
//!
//! # fn main() -> Result<(), huawei_lte_api::Error> {
//! let connection = Connection::new("http://192.168.8.1/", Some("admin"), Some("password"))?;
//! let client = Client::new(&connection);
//! let info = client.device().information()?;
//! println!("Device: {info}");
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod config;
pub mod connection;
pub mod enums;
pub mod errors;
pub mod session;
pub mod tools;
pub mod user;
pub mod usermanual;
pub mod xml;

#[cfg(test)]
pub mod testsupport;

#[cfg(test)]
mod user_test;

mod client;
pub use client::Client;

pub use connection::Connection;
pub use errors::{Error, Result};

/// Library version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
