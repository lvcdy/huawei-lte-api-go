//! Security API group (`api/Security.py`).
//!
//! Firewall, MAC/IP/URL filtering, NAT and port-forwarding.

use serde_json::Value;

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::{get_value, post_set};

/// Security API group.
pub struct Security<'a> {
    conn: &'a Connection,
}

impl<'a> Security<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Security { conn }
    }

    /// `security/bridgemode`. Bridge mode state.
    pub fn bridgemode(&self) -> Result<Value> {
        get_value(self.conn, "api/security/bridgemode")
    }

    /// `security/firewall-switch`. Firewall switch state.
    pub fn get_firewall_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/security/firewall-switch")
    }

    /// Toggle the individual firewall filters.
    pub fn set_firewall_switch(
        &self,
        firewall: bool,
        ip_filter: bool,
        wan_ping_filter: bool,
        url_filter: bool,
        mac_filter: bool,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            (
                "FirewallMainSwitch",
                if firewall {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            (
                "FirewallIPFilterSwitch",
                if ip_filter {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            (
                "FirewallWanPortPingSwitch",
                if wan_ping_filter {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            (
                "firewallurlfilterswitch",
                if url_filter {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            (
                "firewallmacfilterswitch",
                if mac_filter {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
        ]);
        post_set(self.conn, "api/security/firewall-switch", &body)
    }

    /// `security/mac-filter`. MAC filter state.
    pub fn mac_filter(&self) -> Result<Value> {
        get_value(self.conn, "api/security/mac-filter")
    }

    /// `security/lan-ip-filter`. LAN IP filter state.
    pub fn lan_ip_filter(&self) -> Result<Value> {
        get_value(self.conn, "api/security/lan-ip-filter")
    }

    /// `security/virtual-servers`. Port-forward (virtual server) rules.
    pub fn virtual_servers(&self) -> Result<Value> {
        get_value(self.conn, "api/security/virtual-servers")
    }

    /// `security/url-filter`. URL filter rules.
    pub fn url_filter(&self) -> Result<Value> {
        get_value(self.conn, "api/security/url-filter")
    }

    /// Replace the URL filter rules.
    ///
    /// * `urlfilters` — same structure as [`Security::url_filter`] returns.
    pub fn set_url_filter(&self, urlfilters: &XmlMap) -> Result<String> {
        post_set(self.conn, "api/security/url-filter", urlfilters)
    }

    /// `security/upnp`. UPnP state.
    pub fn upnp(&self) -> Result<Value> {
        get_value(self.conn, "api/security/upnp")
    }

    /// Toggle UPnP.
    pub fn set_upnp(&self, enabled: bool) -> Result<String> {
        let body: XmlMap = map_of([(
            "UpnpStatus",
            if enabled {
                "1".to_string()
            } else {
                "0".to_string()
            },
        )]);
        post_set(self.conn, "api/security/upnp", &body)
    }

    /// `security/dmz`. DMZ state.
    pub fn dmz(&self) -> Result<Value> {
        get_value(self.conn, "api/security/dmz")
    }

    /// Enable/disable DMZ and set the exposed host's IP.
    pub fn set_dmz(&self, enabled: bool, ip_address: &str) -> Result<String> {
        let body: XmlMap = map_of([
            (
                "DmzStatus",
                if enabled {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("DmzIPAddress", ip_address.to_string()),
        ]);
        post_set(self.conn, "api/security/dmz", &body)
    }

    /// `security/sip`. SIP ALG state.
    pub fn sip(&self) -> Result<Value> {
        get_value(self.conn, "api/security/sip")
    }

    /// Enable/disable the SIP application-layer gateway and set its port.
    pub fn set_sip(&self, enabled: bool, port: i64) -> Result<String> {
        let body: XmlMap = map_of([
            (
                "SipStatus",
                if enabled {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            ),
            ("SipPort", port.to_string()),
        ]);
        post_set(self.conn, "api/security/sip", &body)
    }

    /// `security/feature-switch`. Feature switch state.
    pub fn feature_switch(&self) -> Result<Value> {
        get_value(self.conn, "api/security/feature-switch")
    }

    /// `security/nat`. NAT state.
    pub fn nat(&self) -> Result<Value> {
        get_value(self.conn, "api/security/nat")
    }

    /// `security/special-applications`. Special application rules.
    pub fn special_applications(&self) -> Result<Value> {
        get_value(self.conn, "api/security/special-applications")
    }

    /// `security/white-lan-ip-filter`. Whitelist LAN IP filter state.
    pub fn white_lan_ip_filter(&self) -> Result<Value> {
        get_value(self.conn, "api/security/white-lan-ip-filter")
    }

    /// `security/white-url-filter`. Whitelist URL filter rules.
    pub fn white_url_filter(&self) -> Result<Value> {
        get_value(self.conn, "api/security/white-url-filter")
    }

    /// Replace the whitelist URL filter rules.
    ///
    /// * `urlfilters` — same structure as [`Security::white_url_filter`] returns.
    pub fn set_white_url_filter(&self, urlfilters: &XmlMap) -> Result<String> {
        post_set(self.conn, "api/security/white-url-filter", urlfilters)
    }

    /// `security/acls` (reverse engineered, likely unused).
    pub fn acls(&self) -> Result<Value> {
        get_value(self.conn, "api/security/acls")
    }

    /// `security/acl`. ACL rule.
    pub fn acl(&self) -> Result<Value> {
        get_value(self.conn, "api/security/acl")
    }
}
