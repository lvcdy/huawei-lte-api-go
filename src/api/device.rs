//! Device API group (`api/Device.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::enums::device::{AntennaType, ControlMode, Mode};
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_get_value, post_set};

/// Device API group.
pub struct Device<'a> {
    conn: &'a Connection,
}

impl<'a> Device<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Device { conn }
    }

    /// `device/information`. Device information.
    pub fn information(&self) -> Result<Value> {
        get_value(self.conn, "api/device/information")
    }

    /// `device/autorun-version`. Autorun version.
    pub fn autorun_version(&self) -> Result<Value> {
        get_value(self.conn, "api/device/autorun-version")
    }

    /// `device/device-feature-switch`. Feature switch status.
    pub fn device_feature_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/device/device-feature-switch")
    }

    /// `device/basic_information`. Basic device information.
    pub fn basic_information(&self) -> Result<Value> {
        get_value(self.conn, "api/device/basic_information")
    }

    /// Restore the basic-information factory defaults.
    pub fn set_basic_information(&self, restore_default_status: bool) -> Result<String> {
        let body: XmlMap = map_of([(
            "restore_default_status",
            if restore_default_status {
                "1".to_string()
            } else {
                "0".to_string()
            },
        )]);
        post_set(self.conn, "api/device/basic_information", &body)
    }

    /// `device/basicinformation`. Basic device information (alternate endpoint).
    pub fn basicinformation(&self) -> Result<Value> {
        get_value(self.conn, "api/device/basicinformation")
    }

    /// `device/usb-tethering-switch`. USB tethering switch status.
    pub fn usb_tethering_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/device/usb-tethering-switch")
    }

    /// `device/boot_time`. Device boot time.
    pub fn boot_time(&self) -> Result<Value> {
        get_value(self.conn, "api/device/boot_time")
    }

    /// Control the device power state (reboot, reset, power off, ...).
    pub fn set_control(&self, control: ControlMode) -> Result<String> {
        let body: XmlMap = map_of([("Control", control.as_i32().to_string())]);
        post_set(self.conn, "api/device/control", &body)
    }

    /// `device/signal`. Signal information.
    pub fn signal(&self) -> Result<Value> {
        get_value(self.conn, "api/device/signal")
    }

    /// `device/antenna_status`. Antenna status.
    pub fn antenna_status(&self) -> Result<Value> {
        get_value(self.conn, "api/device/antenna_status")
    }

    /// `device/antenna_settings`. Antenna settings.
    pub fn get_antenna_settings(&self) -> Result<Value> {
        get_value(self.conn, "api/device/antenna_settings")
    }

    /// Configure the antenna type.
    pub fn set_antenna_settings(&self, antenna_type: AntennaType) -> Result<String> {
        let body: XmlMap = map_of([("antenna_type", antenna_type.as_i32().to_string())]);
        post_set(self.conn, "api/device/antenna_settings", &body)
    }

    /// `device/antenna_type`. Antenna type.
    pub fn antenna_type(&self) -> Result<Value> {
        get_value(self.conn, "api/device/antenna_type")
    }

    /// `device/antenna_set_type`. Antenna set type.
    pub fn antenna_set_type(&self) -> Result<Value> {
        get_value(self.conn, "api/device/antenna_set_type")
    }

    /// `device/logsetting` (reverse engineered, unknown purpose).
    pub fn logsetting(&self) -> Result<Value> {
        get_value(self.conn, "api/device/logsetting")
    }

    /// `device/logport`. Log port.
    pub fn logport(&self) -> Result<Value> {
        get_value(self.conn, "api/device/logport")
    }

    /// `device/datalock`. Data lock status.
    pub fn datalock(&self) -> Result<Value> {
        get_value(self.conn, "api/device/datalock")
    }

    /// `device/vendorname` (POST). Vendor name; may break unsupported devices.
    pub fn vendorname(&self, lang: &str) -> Result<Value> {
        let body: XmlMap = map_of([("language", lang.to_string())]);
        post_get_value(self.conn, "api/device/vendorname", &body)
    }

    /// Set the device mode (debug / telnet / production).
    pub fn mode(&self, mode: Mode) -> Result<String> {
        let body: XmlMap = map_of([("mode", mode.as_i32().to_string())]);
        post_set(self.conn, "api/device/mode", &body)
    }

    /// `device/compresslogfile`. Link to the archived log file.
    pub fn compress_logfile(&self) -> Result<Value> {
        get_value(self.conn, "api/device/compresslogfile")
    }

    /// `device/seccellinfo`. 5G secondary (carrier-aggregation) cell info.
    ///
    /// Supplementary endpoint from
    /// Brovi-Huawei-5G-CPE-Manager.
    pub fn get_sec_cell_info(&self) -> Result<Value> {
        get_value(self.conn, "api/device/seccellinfo")
    }

    /// `device/nbrcellinfo`. 5G neighbour-cell info.
    ///
    /// Supplementary endpoint from
    /// Brovi-Huawei-5G-CPE-Manager.
    pub fn get_nbr_cell_info(&self) -> Result<Value> {
        get_value(self.conn, "api/device/nbrcellinfo")
    }
}
