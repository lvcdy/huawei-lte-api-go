//! OnlineUpdate API group (`api/OnlineUpdate.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_set};

/// OnlineUpdate API group.
pub struct OnlineUpdate<'a> {
    conn: &'a Connection,
}

impl<'a> OnlineUpdate<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        OnlineUpdate { conn }
    }

    /// `online-update/check-new-version`. Check for a new firmware version.
    pub fn check_new_version(&self) -> Result<Value> {
        get_value(self.conn, "api/online-update/check-new-version")
    }

    /// Trigger a check for a new firmware version.
    pub fn set_check_new_version(&self) -> Result<String> {
        let body = XmlMap::new();
        post_set(self.conn, "api/online-update/check-new-version", &body)
    }

    /// `online-update/status`. Online update status.
    pub fn status(&self) -> Result<Value> {
        get_value(self.conn, "api/online-update/status")
    }

    /// `online-update/url-list`. List of URLs for the online update.
    pub fn url_list(&self) -> Result<Value> {
        get_value(self.conn, "api/online-update/url-list")
    }

    /// `online-update/ack-newversion`. Acknowledge the new firmware version.
    pub fn ack_newversion(&self) -> Result<Value> {
        get_value(self.conn, "api/online-update/ack-newversion")
    }

    /// Acknowledge the new firmware version.
    pub fn set_ack_newversion(&self) -> Result<String> {
        let body: XmlMap = map_of([("userAckNewVersion", "0".to_string())]);
        post_set(self.conn, "api/online-update/ack-newversion", &body)
    }

    /// `online-update/cancel-downloading`. Cancel the firmware download.
    pub fn cancel_downloading(&self) -> Result<Value> {
        get_value(self.conn, "api/online-update/cancel-downloading")
    }

    /// Cancel the firmware download.
    pub fn set_cancel_downloading(&self) -> Result<String> {
        let body = XmlMap::new();
        post_set(self.conn, "api/online-update/cancel-downloading", &body)
    }

    /// `online-update/upgrade-messagebox`. Upgrade message box.
    pub fn upgrade_messagebox(&self) -> Result<Value> {
        get_value(self.conn, "api/online-update/upgrade-messagebox")
    }

    /// Set the upgrade message box.
    pub fn set_upgrade_messagebox(&self, messagebox: &str) -> Result<String> {
        let body: XmlMap = map_of([("messagebox", messagebox.to_string())]);
        post_set(self.conn, "api/online-update/upgrade-messagebox", &body)
    }

    /// `online-update/configuration`. Online update configuration.
    pub fn configuration(&self) -> Result<Value> {
        get_value(self.conn, "api/online-update/configuration")
    }

    /// `online-update/autoupdate-config`. Auto-update configuration.
    pub fn autoupdate_config(&self) -> Result<Value> {
        get_value(self.conn, "api/online-update/autoupdate-config")
    }

    /// Set the auto-update configuration.
    ///
    /// * `autoupdate` — enable/disable automatic updates.
    pub fn set_autoupdate_config(&self, autoupdate: bool) -> Result<String> {
        let body: XmlMap = map_of([
            (
                "auto_update",
                if autoupdate {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("ui_download", "0".to_string()),
        ]);
        post_set(self.conn, "api/online-update/autoupdate-config", &body)
    }

    /// `online-update/redirect_cancel`. Cancel the redirection.
    pub fn redirect_cancel(&self) -> Result<Value> {
        get_value(self.conn, "api/online-update/redirect_cancel")
    }
}
