//! Utility helpers mirroring the Python library's `Tools.py`.
//!
//! These functions are used across the API and session layers:
//!
//! * [`rsa_encrypt`] — the RSA encryption used for passwords/tokens.
//! * [`enforce_list_response`] — wrap a single map into a one-element list so
//!   "list" endpoints always return a list, matching the Python helper.
//! * [`strip_dict`] / [`strip_dict_strings`] — remove empty string values.
//! * [`filter_iter`] — filter an iterable by a predicate.
//! * [`datetime_to_epoch`] / [`epoch_to_datetime`] — convert between timestamps
//!   and the `%Y-%m-%d %H:%M:%S` format used by the CPE.

use base64::Engine as _;
use rsa::BigUint;

use crate::errors::{Error, Result};
use crate::xml::{XmlMap, XmlValue};

/// RSA encrypt `data` for the CPE's public key.
///
/// Mirrors `Tools.rsa_encrypt(rsa_e, rsa_n, data, rsa_padding)`:
///
/// * `rsa_e` / `rsa_n` are the public key exponent/modulus as hex strings,
/// * the plaintext is base64-encoded,
/// * chunked into blocks of 245 bytes (PKCS#1 v1.5, `padding == 0`) or
///   214 bytes (PKCS#1 OAEP, `padding == 1`),
/// * each chunk is encrypted with the public key,
/// * the concatenated ciphertext is hexlified,
/// * the result is prefixed with `"0"` if its hex length is odd.
pub fn rsa_encrypt(rsa_e: &str, rsa_n: &str, data: &[u8], rsa_padding: u8) -> Result<String> {
    let e = BigUint::parse_bytes(rsa_e.trim().trim_start_matches("0x").as_bytes(), 16)
        .ok_or_else(|| Error::NoPublicKey(format!("invalid rsa exponent: {rsa_e}")))?;
    let n = BigUint::parse_bytes(rsa_n.trim().trim_start_matches("0x").as_bytes(), 16)
        .ok_or_else(|| Error::NoPublicKey(format!("invalid rsa modulus: {rsa_n}")))?;
    let key = rsa::RsaPublicKey::new(n, e)
        .map_err(|err| Error::NoPublicKey(format!("invalid rsa key: {err}")))?;

    let b64 = base64::engine::general_purpose::STANDARD.encode(data);

    let block_size = match rsa_padding {
        0 => 245usize, // PKCS1_v1_5
        1 => 214usize, // PKCS1_OAEP
        other => {
            return Err(Error::Unexpected(format!(
                "unknown rsa_padding value {other}"
            )))
        }
    };

    let mut rng = rand::thread_rng();
    let mut encrypted = Vec::new();

    for chunk in b64.as_bytes().chunks(block_size) {
        let block = match rsa_padding {
            0 => key
                .encrypt(&mut rng, rsa::Pkcs1v15Encrypt, chunk)
                .map_err(|e| Error::Rsa(format!("encrypt failed: {e}")))?,
            1 => key
                .encrypt(&mut rng, rsa::Oaep::new::<sha1::Sha1>(), chunk)
                .map_err(|e| Error::Rsa(format!("encrypt failed: {e}")))?,
            _ => unreachable!(),
        };
        encrypted.extend_from_slice(&block);
    }

    let hexed = hex::encode(&encrypted);
    if hexed.len() % 2 == 1 {
        Ok(format!("0{hexed}"))
    } else {
        Ok(hexed)
    }
}

/// Ensure a parsed response value behaves like a list, matching
/// `Tools.enforce_list_response`. A single map is wrapped into a one-element
/// list; a list passes through; anything else becomes an empty list.
pub fn enforce_list_response(value: Option<&XmlValue>) -> Vec<XmlValue> {
    match value {
        Some(XmlValue::List(items)) => items.clone(),
        Some(XmlValue::Map(_)) => vec![value.cloned().unwrap()],
        Some(XmlValue::Text(t)) => {
            if t.is_empty() {
                Vec::new()
            } else {
                vec![value.cloned().unwrap()]
            }
        }
        Some(XmlValue::Empty) => Vec::new(),
        None => Vec::new(),
    }
}

/// Return a new map without keys whose values are empty strings.
pub fn strip_dict(map: &XmlMap) -> XmlMap {
    map.iter()
        .filter(|(_, v)| !matches!(v, XmlValue::Text(t) if t.is_empty()))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

/// Return a new map where *all* string values are stripped of surrounding
/// whitespace (used by the Python `User.get_sms_count`-style helpers).
pub fn strip_dict_strings(map: &XmlMap) -> XmlMap {
    map.iter()
        .map(|(k, v)| {
            let v = match v {
                XmlValue::Text(t) => XmlValue::Text(t.trim().to_string()),
                other => other.clone(),
            };
            (k.clone(), v)
        })
        .collect()
}

/// Generic iterator filter that also yields `(index, item)` pairs.
pub fn filter_iter<T>(iter: impl Iterator<Item = T>, predicate: impl Fn(&T) -> bool) -> Vec<T> {
    iter.filter(|item| predicate(item)).collect()
}

/// Convert a `dd-MM-yyyy-HH-mm-ss` CPE timestamp to a UNIX epoch seconds
/// string (matching Python `datetime_to_epoch`).
pub fn datetime_to_epoch(datetime: &str) -> String {
    parse_cpe_datetime(datetime)
        .map(|d| d.and_utc().timestamp().to_string())
        .unwrap_or_default()
}

/// Convert a UNIX epoch (seconds, as a string) to a `dd-MM-yyyy-HH-mm-ss`
/// CPE timestamp (matching Python `epoch_to_datetime`).
pub fn epoch_to_datetime(epoch: &str) -> String {
    let seconds: i64 = epoch.parse().unwrap_or(0);
    let dt = chrono::DateTime::from_timestamp(seconds, 0)
        .map(|d| d.naive_utc())
        .unwrap_or_default();
    dt.format("%d-%m-%Y-%H-%M-%S").to_string()
}

fn parse_cpe_datetime(datetime: &str) -> Option<chrono::NaiveDateTime> {
    chrono::NaiveDateTime::parse_from_str(datetime, "%d-%m-%Y-%H-%M-%S").ok()
}

/// Attempt to coerce a `XmlValue` into an integer, returning `0` on failure
/// (matching the lenient Python `int(x)` conversions used across the API).
pub fn value_to_i64(value: Option<&XmlValue>) -> i64 {
    match value {
        Some(XmlValue::Text(t)) => t.trim().parse().unwrap_or(0),
        Some(XmlValue::Empty) | None => 0,
        _ => 0,
    }
}

/// Attempt to coerce a `XmlValue` into a boolean (truthy if non-empty text
/// and not equal to `"0"`), matching Python truthiness for CPE responses.
pub fn value_to_bool(value: Option<&XmlValue>) -> bool {
    match value {
        Some(XmlValue::Text(t)) => !t.is_empty() && t != "0",
        Some(XmlValue::Empty) | None => false,
        _ => false,
    }
}

/// Build a `BTreeMap` from a list of `(key, value)` pairs.
pub fn map_of<K: Into<String>, V: Into<String>>(pairs: impl IntoIterator<Item = (K, V)>) -> XmlMap {
    pairs
        .into_iter()
        .map(|(k, v)| (k.into(), XmlValue::Text(v.into())))
        .collect()
}

/// Convenience helper to build an XML `<request>` body from a flat field map.
///
/// Wraps [`crate::xml::to_xml`] so API groups can write:
///
/// ```ignore
/// let body = request_body(&[("Username", "admin"), ("Password", "xxx")]);
/// ```
pub fn request_body(pairs: &[(&str, &str)]) -> String {
    let map = pairs
        .iter()
        .map(|(k, v)| (k.to_string(), XmlValue::Text(v.to_string())))
        .collect::<XmlMap>();
    crate::xml::to_xml(&map)
}
