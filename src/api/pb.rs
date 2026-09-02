//! Pb API group (`api/Pb.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::{XmlMap, XmlValue};

use super::{post_get_value, post_set};

/// Pb API group.
pub struct Pb<'a> {
    conn: &'a Connection,
}

impl<'a> Pb<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Pb { conn }
    }

    /// Find a number in the phone book.
    pub fn get_pb_match(&self, phone_number: &str) -> Result<Value> {
        let body: XmlMap = map_of([("Phone", phone_number.to_string())]);
        post_get_value(self.conn, "api/pb/pb-match", &body)
    }

    /// Get the list of phone book entries.
    #[allow(clippy::too_many_arguments)]
    pub fn get_pb_list(
        &self,
        page: i64,
        key_word: &str,
        group_id: i64,
        read_count: i64,
        save_type: i64,
        sort_type: i64,
        ascending: i64,
    ) -> Result<Value> {
        let body: XmlMap = map_of([
            ("GroupID", group_id.to_string()),
            ("PageIndex", page.to_string()),
            ("ReadCount", read_count.to_string()),
            ("SaveType", save_type.to_string()),
            ("SortType", sort_type.to_string()),
            ("Ascending", ascending.to_string()),
            ("KeyWord", key_word.to_string()),
        ]);
        post_get_value(self.conn, "api/pb/pb-list", &body)
    }

    /// Count of phone book entries (reverse engineered).
    pub fn pb_count(&self) -> Result<Value> {
        let body = XmlMap::new();
        post_get_value(self.conn, "api/pb/pb-count", &body)
    }

    /// Count of phone book groups (reverse engineered).
    pub fn group_count(&self) -> Result<Value> {
        let body = XmlMap::new();
        post_get_value(self.conn, "api/pb/group-count", &body)
    }

    /// Add a new entry to the global phone book.
    #[allow(clippy::too_many_arguments)]
    pub fn pb_new(
        &self,
        group_id: i64,
        save_type: i64,
        name: &str,
        mobile_phone: &str,
        home_phone: &str,
        work_phone: &str,
        work_email: &str,
    ) -> Result<String> {
        let mut field_maps = Vec::new();
        for (field_name, value) in [
            ("FormattedName", name),
            ("MobilePhone", mobile_phone),
            ("HomePhone", home_phone),
            ("WorkPhone", work_phone),
            ("WorkEmail", work_email),
        ] {
            let mut f = XmlMap::new();
            f.insert("Name".into(), XmlValue::Text(field_name.to_string()));
            f.insert("Value".into(), XmlValue::Text(value.to_string()));
            field_maps.push(XmlValue::Map(f));
        }

        let mut body = XmlMap::new();
        body.insert("GroupID".into(), XmlValue::Text(group_id.to_string()));
        body.insert("SaveType".into(), XmlValue::Text(save_type.to_string()));
        body.insert("Field".into(), XmlValue::List(field_maps));
        post_set(self.conn, "api/pb/pb-new", &body)
    }

    /// Delete a phone book entry by its index.
    pub fn pb_delete(&self, pb_index: i64) -> Result<String> {
        let body: XmlMap = map_of([("Index", pb_index.to_string())]);
        post_set(self.conn, "api/pb/pb-delete", &body)
    }

    /// Delete a phone book group by its ID.
    pub fn group_delete(&self, group_id: i64) -> Result<String> {
        let body: XmlMap = map_of([("GroupID", group_id.to_string())]);
        post_set(self.conn, "api/pb/group-delete", &body)
    }

    /// Get the list of phone book groups.
    pub fn group_list(
        &self,
        page: i64,
        read_count: i64,
        sort_type: i64,
        ascending: i64,
    ) -> Result<Value> {
        let body: XmlMap = map_of([
            ("PageIndex", page.to_string()),
            ("ReadCount", read_count.to_string()),
            ("SortType", sort_type.to_string()),
            ("Ascending", ascending.to_string()),
        ]);
        post_get_value(self.conn, "api/pb/group-list", &body)
    }

    /// Create a new phone book group by name.
    pub fn group_new(&self, name_str: &str) -> Result<String> {
        let body: XmlMap = map_of([("GroupName", name_str.to_string())]);
        post_set(self.conn, "api/pb/group-new", &body)
    }
}
