//! Lightweight XML builder/parser used to talk to the Huawei CPE.
//!
//! The Python library uses `xmltodict` to turn a Python dict into XML for
//! request bodies (`<request>...</request>`) and to parse XML responses back
//! into dicts. This module re-implements exactly that behaviour in Rust:
//!
//! * [`XmlValue`] is a node in the tree (a scalar string, a nested map, or a
//!   list of nodes).
//! * [`XmlMap`] is a `BTreeMap<String, XmlValue>` — ordered, which matters
//!   because some devices are sensitive to the order of the fields
//!   (see `sms/sms-list` in the Python library).
//! * [`to_xml`] serializes a map into the `<request>` XML document.
//! * [`from_xml`] parses an XML response into an `XmlMap`, collapsing
//!   single-child elements and producing lists for repeated elements, exactly
//!   like `xmltodict`.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::errors::{Error, Result};

/// An ordered XML attribute map (field name → value).
pub type XmlMap = BTreeMap<String, XmlValue>;

/// A node in an XML tree, mirroring what `xmltodict` produces.
#[derive(Debug, Clone, PartialEq)]
pub enum XmlValue {
    /// A leaf: text content (always a string, like Python).
    Text(String),
    /// A nested element with children.
    Map(XmlMap),
    /// A repeated element (Python `xmltodict` produces a list).
    List(Vec<XmlValue>),
    /// Explicit empty element (`<tag/>`).
    Empty,
}

impl XmlValue {
    /// Coerce to a string, treating `None`/`Empty` as empty string, like the
    /// Python code `str(data.get(key, ""))`.
    pub fn as_str(&self) -> String {
        match self {
            XmlValue::Text(s) => s.clone(),
            XmlValue::Empty => String::new(),
            XmlValue::Map(_) | XmlValue::List(_) => String::new(),
        }
    }

    /// Try to parse as an integer (default 0 on failure), like the Python
    /// `int(data.get(key, 0))` patterns.
    pub fn as_int(&self) -> i64 {
        self.as_str().trim().parse().unwrap_or(0)
    }

    /// True if this value represents the string `"1"` (used for boolean flags).
    pub fn is_truthy(&self) -> bool {
        self.as_str() == "1"
    }
}

/// Escape a string for use inside XML text content.
pub fn escape_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

/// Serialize a single `XmlValue` under `tag` into `writer`.
///
/// Lists are emitted as repeated `<tag>...</tag>` elements (matching
/// `xmltodict`'s handling of a Python list).
fn write_value(tag: &str, value: &XmlValue, depth: usize, out: &mut String) {
    let indent = "  ".repeat(depth);
    match value {
        XmlValue::Empty => {
            let _ = writeln!(out, "{indent}<{tag}/>");
        }
        XmlValue::Text(text) => {
            let _ = writeln!(out, "{indent}<{tag}>{}</{tag}>", escape_text(text));
        }
        XmlValue::Map(map) => {
            let _ = writeln!(out, "{indent}<{tag}>");
            for (child_tag, child) in map {
                write_value(child_tag, child, depth + 1, out);
            }
            let _ = writeln!(out, "{indent}</{tag}>");
        }
        XmlValue::List(items) => {
            for item in items {
                write_value(tag, item, depth, out);
            }
        }
    }
}

/// Serialize an `XmlMap` into a `<request>` XML document (UTF-8).
///
/// The result is later encoded to CESU-8 by the session layer (mirroring
/// `cesu8_encode(xmltodict.unparse(...))` in Python).
pub fn to_xml(data: &XmlMap) -> String {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<request>\n");
    for (tag, value) in data {
        write_value(tag, value, 1, &mut out);
    }
    out.push_str("</request>");
    out
}

/// Parse a UTF-8 XML document into an `XmlMap`, mirroring `xmltodict`:
///
/// * A leaf element `<Name>x</Name>` becomes `{"Name": Text("x")}`.
/// * An element with children `<A><B>1</B></A>` becomes
///   `{"A": Map({"B": Text("1")})}`.
/// * Repeated sibling elements become a `List`.
/// * An element with attributes keeps them under `@attr` keys and its text
///   under `#text` (rare in Huawei responses).
///
/// The returned map has the document root element as its single key, e.g.
/// `{"response": {...}}` — exactly like `xmltodict.parse`.
pub fn from_xml(input: &[u8]) -> Result<XmlMap> {
    let text = std::str::from_utf8(input)
        .map_err(|e| Error::Protocol(format!("invalid UTF-8 in XML response: {e}")))?;
    let mut reader = quick_xml::Reader::from_str(text);
    reader.config_mut().trim_text(false);

    // Stack of (tag, children) for the elements we are currently inside.
    let mut stack: Vec<(String, XmlMap)> = Vec::new();
    let mut root: Option<XmlMap> = None;

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                stack.push((name, XmlMap::new()));
            }
            Ok(quick_xml::events::Event::Empty(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut children = XmlMap::new();
                for attr in e.attributes().flatten() {
                    let key = format!("@{}", String::from_utf8_lossy(attr.key.as_ref()));
                    children.insert(
                        key,
                        XmlValue::Text(
                            attr.unescape_value()
                                .map_err(|err| {
                                    Error::Protocol(format!("XML attribute unescape error: {err}"))
                                })?
                                .to_string(),
                        ),
                    );
                }
                if children.is_empty() {
                    if let Some((_, parent)) = stack.last_mut() {
                        insert(parent, &name, XmlValue::Empty);
                    } else {
                        let mut m = XmlMap::new();
                        m.insert(name, XmlValue::Empty);
                        root = Some(m);
                    }
                } else {
                    let child = XmlValue::Map(children);
                    if let Some((_, parent)) = stack.last_mut() {
                        insert(parent, &name, child);
                    } else {
                        let mut m = XmlMap::new();
                        m.insert(name, child);
                        root = Some(m);
                    }
                }
            }
            Ok(quick_xml::events::Event::Text(e)) => {
                let t = e
                    .unescape()
                    .map_err(|e| Error::Protocol(format!("XML text unescape error: {e}")))?;
                let t = t.trim().to_string();
                if !t.is_empty() {
                    if let Some((_, children)) = stack.last_mut() {
                        insert(children, "#text", XmlValue::Text(t));
                    }
                }
            }
            Ok(quick_xml::events::Event::End(_)) => {
                if let Some((tag, children)) = stack.pop() {
                    let child = fold_element(children);
                    if let Some((_, parent_children)) = stack.last_mut() {
                        insert(parent_children, &tag, child);
                    } else {
                        // We popped the root element.
                        let mut m = XmlMap::new();
                        m.insert(tag, child);
                        root = Some(m);
                    }
                }
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => {
                return Err(Error::Xml(e));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(root.unwrap_or_default())
}

/// Collapse a parsed element's children into an `XmlValue`:
///
/// * `{ "#text": Text("x") }` (no attribute children) → `Text("x")`
/// * empty → `Empty`
/// * anything else (has element children or attributes) → `Map`
fn fold_element(children: XmlMap) -> XmlValue {
    let has_attrs = children.keys().any(|k| k.starts_with('@'));
    if !has_attrs {
        if let Some(XmlValue::Text(t)) = children.get("#text") {
            if children.len() == 1 {
                return XmlValue::Text(t.clone());
            }
        }
        // Mixed text + child elements: keep structure, drop the stray text.
        let mut map = children;
        map.remove("#text");
        if map.is_empty() {
            return XmlValue::Empty;
        }
        return XmlValue::Map(map);
    }
    XmlValue::Map(children)
}

/// Convert a parsed JSON value into an `XmlMap`, so JSON endpoints (rare) can
/// be consumed with the same [`MapExt`] helpers as XML ones.
pub fn json_to_map(value: &serde_json::Value) -> XmlValue {
    match value {
        serde_json::Value::Object(obj) => {
            let mut map = XmlMap::new();
            for (k, v) in obj {
                map.insert(k.clone(), json_to_map(v));
            }
            XmlValue::Map(map)
        }
        serde_json::Value::Array(arr) => XmlValue::List(arr.iter().map(json_to_map).collect()),
        serde_json::Value::String(s) => XmlValue::Text(s.clone()),
        serde_json::Value::Number(n) => XmlValue::Text(n.to_string()),
        serde_json::Value::Bool(b) => XmlValue::Text(if *b { "1".into() } else { "0".into() }),
        serde_json::Value::Null => XmlValue::Empty,
    }
}

/// Insert a value into a map, converting repeated keys into a `List`.
fn insert(map: &mut XmlMap, tag: &str, value: XmlValue) {
    match map.get_mut(tag) {
        Some(XmlValue::List(items)) => items.push(value),
        Some(existing) => {
            let prev = existing.clone();
            *existing = XmlValue::List(vec![prev, value]);
        }
        None => {
            map.insert(tag.to_string(), value);
        }
    }
}

/// Convenience accessors used across API groups.
pub trait MapExt {
    /// Get a nested map by dot-path, e.g. `get_map(&["response", "Ssids"])`.
    fn get_map(&self, path: &[&str]) -> Option<&XmlMap>;
    /// Get a string value by dot-path.
    fn get_str(&self, path: &[&str]) -> Option<String>;
}

impl MapExt for XmlMap {
    fn get_map(&self, path: &[&str]) -> Option<&XmlMap> {
        let mut cur: &XmlValue = self.get(path[0])?;
        for key in &path[1..] {
            match cur {
                XmlValue::Map(m) => cur = m.get(*key)?,
                _ => return None,
            }
        }
        match cur {
            XmlValue::Map(m) => Some(m),
            _ => None,
        }
    }

    fn get_str(&self, path: &[&str]) -> Option<String> {
        let mut cur: &XmlValue = self.get(path[0])?;
        for key in &path[1..] {
            match cur {
                XmlValue::Map(m) => cur = m.get(*key)?,
                _ => return None,
            }
        }
        Some(cur.as_str())
    }
}

/// Convert an [`XmlValue`] into a [`serde_json::Value`].
///
/// This is the bridge used to build strongly-typed response structs: API
/// methods deserialize their `XmlMap` response (or a child of it) into a
/// `#[derive(serde::Deserialize)]` struct via
/// `serde_json::from_value(map_to_json(map))`.
pub fn map_to_json(map: &XmlMap) -> serde_json::Value {
    let mut obj = serde_json::Map::with_capacity(map.len());
    for (k, v) in map {
        obj.insert(k.clone(), value_to_json(v));
    }
    serde_json::Value::Object(obj)
}

/// Convert a single [`XmlValue`] into a [`serde_json::Value`].
pub fn value_to_json(value: &XmlValue) -> serde_json::Value {
    match value {
        XmlValue::Text(s) => serde_json::Value::String(s.clone()),
        XmlValue::Empty => serde_json::Value::Null,
        XmlValue::Map(m) => map_to_json(m),
        XmlValue::List(items) => {
            serde_json::Value::Array(items.iter().map(value_to_json).collect())
        }
    }
}

/// Deserialize an `XmlMap` (or the `response` child of it) into a typed
/// struct `T`, returning a [`crate::errors::Error`] on failure.
///
/// If a `response` key is present and is a map, it is used; otherwise the
/// whole map is used. Missing fields deserialize to their serde defaults /
/// `None` for `Option` fields, so a partial or unknown schema does not panic.
pub fn deserialize_response<T>(map: &XmlMap) -> crate::errors::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let value = match map.get("response") {
        Some(XmlValue::Map(m)) => map_to_json(m),
        _ => map_to_json(map),
    };
    serde_json::from_value::<T>(value).map_err(|e| {
        crate::errors::Error::Other(format!("failed to deserialize API response: {e}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_xml_simple() {
        let mut m = XmlMap::new();
        m.insert("Username".into(), XmlValue::Text("admin".into()));
        m.insert("Password".into(), XmlValue::Text("c2VjcmV0".into()));
        let xml = to_xml(&m);
        assert!(xml.contains("<Username>admin</Username>"));
        assert!(xml.contains("<Password>c2VjcmV0</Password>"));
        assert!(xml.starts_with("<?xml"));
    }

    #[test]
    fn test_to_xml_nested_and_list() {
        let mut inner = XmlMap::new();
        inner.insert("Index".into(), XmlValue::Text("1".into()));
        let mut m = XmlMap::new();
        m.insert("Ssids".into(), XmlValue::Map(inner.clone()));
        m.insert(
            "Ssid".into(),
            XmlValue::List(vec![
                XmlValue::Map(inner.clone()),
                XmlValue::Map(inner.clone()),
            ]),
        );
        let xml = to_xml(&m);
        assert_eq!(xml.matches("<Ssid>").count(), 2);
    }

    #[test]
    fn test_escape() {
        assert_eq!(escape_text("a<b&c"), "a&lt;b&amp;c");
    }

    #[test]
    fn test_from_xml_leaf_folds_to_text() {
        let xml = br#"<?xml version="1.0" encoding="UTF-8"?><response><DeviceName>HG8121C</DeviceName></response>"#;
        let map = from_xml(xml).unwrap();
        let resp = map.get_map(&["response"]).expect("response key");
        assert_eq!(resp.get_str(&["DeviceName"]).as_deref(), Some("HG8121C"));
        assert!(matches!(
            resp.get("DeviceName"),
            Some(XmlValue::Text(t)) if t == "HG8121C"
        ));
    }

    #[test]
    fn test_from_xml_nested_map() {
        let xml =
            br#"<response><DeviceInfo><Name>A</Name><Class>B</Class></DeviceInfo></response>"#;
        let map = from_xml(xml).unwrap();
        let resp = map.get_map(&["response"]).unwrap();
        let info = resp.get_map(&["DeviceInfo"]).unwrap();
        assert_eq!(info.get_str(&["Name"]).as_deref(), Some("A"));
        assert_eq!(info.get_str(&["Class"]).as_deref(), Some("B"));
    }

    #[test]
    fn test_from_xml_repeated_becomes_list() {
        let xml = br#"<response><SmsList><Index>1</Index><Index>2</Index><Index>3</Index></SmsList></response>"#;
        let map = from_xml(xml).unwrap();
        let resp = map.get_map(&["response"]).unwrap();
        let list = resp.get("SmsList").unwrap();
        match list {
            XmlValue::Map(m) => match m.get("Index").unwrap() {
                XmlValue::List(items) => assert_eq!(items.len(), 3),
                other => panic!("expected list, got {other:?}"),
            },
            other => panic!("expected map, got {other:?}"),
        }
    }

    #[test]
    fn test_from_xml_empty_element() {
        let xml = br#"<response><Empty/></response>"#;
        let map = from_xml(xml).unwrap();
        let resp = map.get_map(&["response"]).unwrap();
        assert!(matches!(resp.get("Empty"), Some(XmlValue::Empty)));
    }

    #[test]
    fn test_roundtrip() {
        let mut inner = XmlMap::new();
        inner.insert("Index".into(), XmlValue::Text("1".into()));
        let mut m = XmlMap::new();
        m.insert("Ssids".into(), XmlValue::Map(inner.clone()));
        m.insert(
            "Ssid".into(),
            XmlValue::List(vec![
                XmlValue::Map(inner.clone()),
                XmlValue::Map(inner.clone()),
            ]),
        );
        let xml = to_xml(&m);
        let parsed = from_xml(xml.as_bytes()).unwrap();
        let req = parsed.get_map(&["request"]).unwrap();
        assert_eq!(req.get_str(&["Ssids", "Index"]).as_deref(), Some("1"));
        match req.get("Ssid").unwrap() {
            XmlValue::List(items) => assert_eq!(items.len(), 2),
            other => panic!("expected list, got {other:?}"),
        }
    }
}
