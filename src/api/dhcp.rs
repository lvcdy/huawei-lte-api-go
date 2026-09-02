//! Dhcp API group (`api/Dhcp.py`).

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_set};

/// Dhcp API group.
pub struct Dhcp<'a> {
    conn: &'a Connection,
}

impl<'a> Dhcp<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Dhcp { conn }
    }

    /// `dhcp/settings`. DHCP settings.
    pub fn settings(&self) -> Result<Value> {
        get_value(self.conn, "api/dhcp/settings")
    }

    /// `dhcp/feature-switch`. DHCP feature switch status.
    pub fn feature_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/dhcp/feature-switch")
    }

    /// `dhcp/dhcp-host-info`. DHCP host information (reverse engineered).
    pub fn dhcp_host_info(&self) -> Result<Value> {
        get_value(self.conn, "api/dhcp/dhcp-host-info")
    }

    /// `dhcp/static-addr-info`. Static address information (reverse engineered).
    pub fn static_addr_info(&self) -> Result<Value> {
        get_value(self.conn, "api/dhcp/static-addr-info")
    }

    /// Configure the DHCP server settings.
    ///
    /// * `dhcp_ip_address` — IP address of the DHCP server.
    /// * `dhcp_start_ip_range` / `dhcp_end_ip_range` — lease IP range offsets
    ///   (the final octets, derived from `dhcp_ip_address`).
    /// * `primary_dns` / `secondary_dns` — optional DNS server IPs.
    #[allow(clippy::too_many_arguments)]
    pub fn set_settings(
        &self,
        dhcp_ip_address: &str,
        dhcp_lan_netmask: &str,
        dhcp_status: bool,
        dhcp_start_ip_range: i64,
        dhcp_end_ip_range: i64,
        dhcp_lease_time: i64,
        dns_status: bool,
        primary_dns: Option<&str>,
        secondary_dns: Option<&str>,
        show_dns_setting: bool,
    ) -> Result<String> {
        let parts: Vec<&str> = dhcp_ip_address.split('.').collect();
        let base = parts[..parts.len() - 1].join(".");
        let dhcp_start_ip_address = format!("{base}.{dhcp_start_ip_range}");
        let dhcp_end_ip_address = format!("{base}.{dhcp_end_ip_range}");

        let body: XmlMap = map_of([
            ("DhcpIPAddress", dhcp_ip_address.to_string()),
            ("DhcpLanNetmask", dhcp_lan_netmask.to_string()),
            (
                "DhcpStatus",
                if dhcp_status {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("DhcpStartIPAddress", dhcp_start_ip_address),
            ("DhcpEndIPAddress", dhcp_end_ip_address),
            ("DhcpLeaseTime", dhcp_lease_time.to_string()),
            (
                "DnsStatus",
                if dns_status {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("PrimaryDns", primary_dns.unwrap_or("").to_string()),
            ("SecondaryDns", secondary_dns.unwrap_or("").to_string()),
            (
                "ShowDnsSetting",
                if show_dns_setting {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
        ]);
        post_set(self.conn, "api/dhcp/settings", &body)
    }
}
