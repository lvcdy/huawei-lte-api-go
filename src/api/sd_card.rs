//! SdCard API group (`api/SdCard.py`).
//!
//! Storage sharing (DLNA / Samba) and SD-card file management.

use chrono::{Datelike, Timelike};
use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::{XmlMap, XmlValue};

use super::{get_value, post_set};

/// SdCard API group.
pub struct SdCard<'a> {
    conn: &'a Connection,
}

impl<'a> SdCard<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        SdCard { conn }
    }

    /// `sdcard/dlna-setting`. DLNA setting.
    pub fn dlna_setting(&self) -> Result<Value> {
        get_value(self.conn, "api/sdcard/dlna-setting")
    }

    /// Set DLNA settings.
    ///
    /// * `enabled` — enable DLNA.
    /// * `share_all` — share the whole card.
    /// * `share_path` — path to share (default `/`).
    pub fn set_dlna_setting(
        &self,
        enabled: bool,
        share_all: bool,
        share_path: &str,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            (
                "enabled",
                if enabled {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("sharepath", share_path.to_string()),
            (
                "shareallpath",
                if share_all {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
        ]);
        post_set(self.conn, "api/sdcard/dlna-setting", &body)
    }

    /// `sdcard/sdcard`. Sharing information.
    pub fn sdcard(&self) -> Result<Value> {
        get_value(self.conn, "api/sdcard/sdcard")
    }

    /// `sdcard/sdcardsamba`. Samba sharing state.
    pub fn sdcardsamba(&self) -> Result<Value> {
        get_value(self.conn, "api/sdcard/sdcardsamba")
    }

    /// Enable file sharing over SMB.
    pub fn set_sdcardsamba(
        &self,
        enabled: bool,
        server_name: &str,
        server_description: &str,
        workgroup_name: &str,
        anonymous_access: bool,
        printer_enabled: bool,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            (
                "enabled",
                if enabled {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("servername", server_name.to_string()),
            ("serverdescription", server_description.to_string()),
            ("workgroupname", workgroup_name.to_string()),
            (
                "anonymousaccess",
                if anonymous_access {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            (
                "printerenable",
                if printer_enabled {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
        ]);
        post_set(self.conn, "api/sdcard/sdcardsamba", &body)
    }

    /// `sdcard/printerlist`. Printer list.
    pub fn printerlist(&self) -> Result<Value> {
        get_value(self.conn, "api/sdcard/printerlist")
    }

    /// `sdcard/share-account`. Share accounts.
    pub fn share_account(&self) -> Result<Value> {
        get_value(self.conn, "api/sdcard/share-account")
    }

    /// `sdcard/sdfile` (reverse engineered).
    pub fn sdfile(&self) -> Result<Value> {
        get_value(self.conn, "api/sdcard/sdfile")
    }

    /// `sdcard/fileupload` (reverse engineered).
    pub fn fileupload(&self) -> Result<Value> {
        get_value(self.conn, "api/sdcard/fileupload")
    }

    /// `sdcard/Check_file_exist` (reverse engineered; note the mixed case).
    pub fn check_file_exist(&self) -> Result<Value> {
        get_value(self.conn, "api/sdcard/Check_file_exist")
    }

    /// Create a directory on the SD card.
    ///
    /// * `name` — directory name to create.
    /// * `current_path` — parent path (default `/`).
    /// * `created` — creation timestamp (defaults to now in UTC).
    pub fn create_dir(
        &self,
        name: &str,
        current_path: &str,
        created: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<String> {
        let created = created.unwrap_or_else(chrono::Utc::now);
        let time: XmlMap = map_of([
            ("Year", created.year().to_string()),
            ("Month", created.month().to_string()),
            ("Day", created.day().to_string()),
            ("Hour", created.hour().to_string()),
            ("Min", created.minute().to_string()),
            ("Sec", created.second().to_string()),
        ]);
        let mut body = XmlMap::new();
        body.insert(
            "CurrentPath".to_string(),
            XmlValue::Text(current_path.to_string()),
        );
        body.insert("FileName".to_string(), XmlValue::Text(name.to_string()));
        body.insert("Time".to_string(), XmlValue::Map(time));
        post_set(self.conn, "api/sdcard/createdir", &body)
    }

    /// Delete a file or directory on the SD card.
    pub fn delete_file(&self, name: &str, current_path: &str) -> Result<String> {
        let body: XmlMap = map_of([
            ("CurrentPath", current_path.to_string()),
            ("DeleteFileList", name.to_string()),
        ]);
        post_set(self.conn, "api/sdcard/deletefile", &body)
    }

    /// `sdcard/sdcapacity`. SD-card capacity information.
    pub fn sd_capacity(&self) -> Result<Value> {
        get_value(self.conn, "api/sdcard/sdcapacity")
    }
}
