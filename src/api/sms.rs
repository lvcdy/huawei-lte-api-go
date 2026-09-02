//! Sms API group (`api/Sms.py`).
//!
//! Reading, sending, saving and configuring SMS on the CPE.

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::{XmlMap, XmlValue};

use super::{get_value, post_get_value, post_set};

/// Sms API group.
pub struct Sms<'a> {
    conn: &'a Connection,
}

impl<'a> Sms<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Sms { conn }
    }

    /// `sms/get-cbsnewslist`. CBS news list.
    pub fn get_cbsnewslist(&self) -> Result<Value> {
        get_value(self.conn, "api/sms/get-cbsnewslist")
    }

    /// `sms/sms-count`. SMS count information.
    pub fn sms_count(&self) -> Result<Value> {
        get_value(self.conn, "api/sms/sms-count")
    }

    /// `sms/splitinfo-sms`. SMS split information.
    pub fn splitinfo_sms(&self) -> Result<Value> {
        get_value(self.conn, "api/sms/splitinfo-sms")
    }

    /// `sms/sms-feature-switch`. SMS feature switch.
    pub fn sms_feature_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/sms/sms-feature-switch")
    }

    /// `sms/send-status`. Last send status.
    pub fn send_status(&self) -> Result<Value> {
        get_value(self.conn, "api/sms/send-status")
    }

    /// Fetch a page of SMS messages.
    ///
    /// * `page` — 1-based page number.
    /// * `box_type` — box type enum as `i64` (e.g. local inbox).
    /// * `read_count` — messages per page.
    /// * `sort_type` — sort type enum as `i64`.
    /// * `ascending` — ascending (vs descending) order.
    /// * `unread_preferred` — order unread messages first.
    pub fn get_sms_list(
        &self,
        page: i64,
        box_type: i64,
        read_count: i64,
        sort_type: i64,
        ascending: bool,
        unread_preferred: bool,
    ) -> Result<Value> {
        let body: XmlMap = map_of([
            ("PageIndex", page.to_string()),
            ("ReadCount", read_count.to_string()),
            ("BoxType", box_type.to_string()),
            ("SortType", sort_type.to_string()),
            (
                "Ascending",
                if ascending {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            (
                "UnreadPreferred",
                if unread_preferred {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
        ]);
        post_get_value(self.conn, "api/sms/sms-list", &body)
    }

    /// Delete a single SMS by its id.
    pub fn delete_sms(&self, sms_id: i64) -> Result<String> {
        let body: XmlMap = map_of([("Index", sms_id.to_string())]);
        post_set(self.conn, "api/sms/delete-sms", &body)
    }

    /// Back up SMS from the SIM.
    ///
    /// * `from_date` — `%Y-%m-%d %H:%M:%S` (or `dd-MM-yyyy-HH-mm-ss`) timestamp.
    /// * `is_move` — move (vs copy) the messages.
    pub fn backup_sim(&self, is_move: bool, from_date: &str) -> Result<String> {
        let body: XmlMap = map_of([
            (
                "IsMove",
                if is_move {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("Date", from_date.to_string()),
        ]);
        post_set(self.conn, "api/sms/backup-sim", &body)
    }

    /// Mark a single SMS as read.
    pub fn set_read(&self, sms_id: i64) -> Result<String> {
        let body: XmlMap = map_of([("Index", sms_id.to_string())]);
        post_set(self.conn, "api/sms/set-read", &body)
    }

    /// Save a composed SMS (without sending).
    ///
    /// * `phone_numbers` — recipient list.
    /// * `message` — SMS body text.
    /// * `sms_index` — index (default `-1`).
    /// * `sca` — message-center number in INTL format, e.g. `+420603052000`.
    /// * `text_mode` — encoding mode enum as `i64`.
    /// * `from_date` — timestamp used for `Date`.
    pub fn save_sms(
        &self,
        phone_numbers: &[String],
        message: &str,
        sms_index: i64,
        sca: Option<&str>,
        text_mode: i64,
        from_date: &str,
    ) -> Result<String> {
        let body = self.sms_body(phone_numbers, message, sms_index, sca, text_mode, from_date);
        post_set(self.conn, "api/sms/save-sms", &body)
    }

    /// Send an SMS.
    ///
    /// * `phone_numbers` — recipient list.
    /// * `message` — SMS body text.
    /// * `sms_index` — index (default `-1`).
    /// * `sca` — message-center number in INTL format, e.g. `+420603052000`.
    /// * `text_mode` — encoding mode enum as `i64`.
    /// * `from_date` — timestamp used for `Date`.
    pub fn send_sms(
        &self,
        phone_numbers: &[String],
        message: &str,
        sms_index: i64,
        sca: Option<&str>,
        text_mode: i64,
        from_date: &str,
    ) -> Result<String> {
        let body = self.sms_body(phone_numbers, message, sms_index, sca, text_mode, from_date);
        post_set(self.conn, "api/sms/send-sms", &body)
    }

    /// Shared request body for [`Sms::save_sms`] / [`Sms::send_sms`].
    fn sms_body(
        &self,
        phone_numbers: &[String],
        message: &str,
        sms_index: i64,
        sca: Option<&str>,
        text_mode: i64,
        from_date: &str,
    ) -> XmlMap {
        let mut phones = XmlMap::new();
        phones.insert(
            "Phone".to_string(),
            XmlValue::List(
                phone_numbers
                    .iter()
                    .map(|p| XmlValue::Text(p.clone()))
                    .collect(),
            ),
        );
        let mut body = XmlMap::new();
        body.insert("Index".to_string(), XmlValue::Text(sms_index.to_string()));
        body.insert("Phones".to_string(), XmlValue::Map(phones));
        body.insert(
            "Sca".to_string(),
            XmlValue::Text(sca.unwrap_or_default().to_string()),
        );
        body.insert("Content".to_string(), XmlValue::Text(message.to_string()));
        body.insert(
            "Length".to_string(),
            XmlValue::Text(message.len().to_string()),
        );
        body.insert(
            "Reserved".to_string(),
            XmlValue::Text(text_mode.to_string()),
        );
        body.insert("Date".to_string(), XmlValue::Text(from_date.to_string()));
        body
    }

    /// `sms/cancel-send`. Cancel a pending send.
    pub fn cancel_send(&self) -> Result<String> {
        // Python sends a bare scalar `1` as the body; the shared `post_set`
        // expects a map, so an empty body is sent instead.
        let body = XmlMap::new();
        post_set(self.conn, "api/sms/cancel-send", &body)
    }

    /// `sms/config`. SMS configuration.
    pub fn config(&self) -> Result<Value> {
        get_value(self.conn, "api/sms/config")
    }

    /// Set the default SMS send configuration.
    ///
    /// * `sca` — message-center number in INTL format.
    /// * `save_mode` — save mode enum as `i64`.
    /// * `validity` — validity in seconds.
    /// * `use_s_report` — request send/receive status reports.
    /// * `send_type` — send type enum as `i64`.
    /// * `priority` — priority enum as `i64`.
    pub fn set_config(
        &self,
        sca: &str,
        save_mode: i64,
        validity: i64,
        use_s_report: bool,
        send_type: i64,
        priority: i64,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            ("SaveMode", save_mode.to_string()),
            ("Validity", validity.to_string()),
            ("Sca", sca.to_string()),
            (
                "UseSReport",
                if use_s_report {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("SendType", send_type.to_string()),
            ("Priority", priority.to_string()),
        ]);
        post_set(self.conn, "api/sms/config", &body)
    }

    /// `sms/sms-count-contact`. Contact SMS count.
    pub fn sms_count_contact(&self) -> Result<Value> {
        get_value(self.conn, "api/sms/sms-count-contact")
    }

    /// Fetch a page of contact SMS.
    pub fn sms_list_contact(&self, pageindex: i64, readcount: i64) -> Result<Value> {
        let body: XmlMap = map_of([
            ("pageindex", pageindex.to_string()),
            ("readcount", readcount.to_string()),
        ]);
        post_get_value(self.conn, "api/sms/sms-list-contact", &body)
    }

    /// Return SMS in PDU format.
    pub fn get_sms_list_pdu(&self, page: i64, box_type: i64, read_count: i64) -> Result<Value> {
        let body: XmlMap = map_of([
            ("PageIndex", page.to_string()),
            ("ReadCount", read_count.to_string()),
            ("BoxType", box_type.to_string()),
        ]);
        post_get_value(self.conn, "api/sms/sms-list-pdu", &body)
    }

    /// `sms/split-sms` (reverse engineered).
    pub fn split_sms(&self) -> Result<Value> {
        get_value(self.conn, "api/sms/split-sms")
    }

    /// Send an SMS in PDU format (not tested on real hardware).
    ///
    /// * `pdu` — PDU to send, e.g. `001100098121436587F900000B05E8329BFD06`.
    /// * `length` — magic PDU length (not `pdu.len()`).
    /// * `sms_index` — index (default `-1`).
    /// * `sca` — message-center number in INTL format.
    /// * `validity` — validity in seconds.
    /// * `status_report` — request a status report.
    /// * `save_mode` — save mode enum as `i64`.
    /// * `send_type` — send type enum as `i64`.
    #[allow(clippy::too_many_arguments)]
    pub fn send_sms_pdu(
        &self,
        pdu: &str,
        length: i64,
        sms_index: i64,
        sca: Option<&str>,
        validity: i64,
        status_report: bool,
        save_mode: i64,
        send_type: i64,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            ("Index", sms_index.to_string()),
            ("PDU", pdu.to_string()),
            ("Length", length.to_string()),
            ("SaveMode", save_mode.to_string()),
            ("Validity", validity.to_string()),
            ("Sca", sca.unwrap_or_default().to_string()),
            (
                "UseSReport",
                if status_report {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("SendType", send_type.to_string()),
        ]);
        post_set(self.conn, "api/sms/send-sms-pdu", &body)
    }

    /// `sms/recover-sms` (reverse engineered).
    pub fn recover_sms(&self) -> Result<Value> {
        get_value(self.conn, "api/sms/recover-sms")
    }

    /// `sms/copy-sms` (reverse engineered).
    pub fn copy_sms(&self) -> Result<Value> {
        get_value(self.conn, "api/sms/copy-sms")
    }

    /// `sms/move-sms` (reverse engineered).
    pub fn move_sms(&self) -> Result<Value> {
        get_value(self.conn, "api/sms/move-sms")
    }
}
