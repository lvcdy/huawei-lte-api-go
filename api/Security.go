package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Security 对应 Security.py。
type Security struct {
	*session.ApiGroup
}

// NewSecurity 创建 Security API 分组。
func NewSecurity(s *session.Session) *Security {
	return &Security{ApiGroup: session.NewApiGroup(s)}
}

// Bridgemode 对应 security/bridgemode。
func (s *Security) Bridgemode() (map[string]interface{}, error) {
	return s.S.Get("security/bridgemode", nil, "api")
}

// GetFirewallSwitch 获取防火墙开关。对应 security/firewall-switch。
func (s *Security) GetFirewallSwitch() (map[string]interface{}, error) {
	return s.S.Get("security/firewall-switch", nil, "api")
}

// SetFirewallSwitch 设置防火墙开关。对应 security/firewall-switch (post_set)。
func (s *Security) SetFirewallSwitch(
	firewall bool,
	ipFilter bool,
	wanPingFilter bool,
	urlFilter bool,
	macFilter bool,
) (interface{}, error) {
	return s.S.PostSet("security/firewall-switch", session.O(
		"FirewallMainSwitch", boolToInt(firewall),
		"FirewallIPFilterSwitch", boolToInt(ipFilter),
		"FirewallWanPortPingSwitch", boolToInt(wanPingFilter),
		"firewallurlfilterswitch", boolToInt(urlFilter),
		"firewallmacfilterswitch", boolToInt(macFilter),
	), false, "api", false, false)
}

// MacFilter 对应 security/mac-filter。
func (s *Security) MacFilter() (map[string]interface{}, error) {
	return s.S.Get("security/mac-filter", nil, "api")
}

// LanIpFilter 对应 security/lan-ip-filter。
func (s *Security) LanIpFilter() (map[string]interface{}, error) {
	return s.S.Get("security/lan-ip-filter", nil, "api")
}

// VirtualServers 对应 security/virtual-servers。
func (s *Security) VirtualServers() (map[string]interface{}, error) {
	return s.S.Get("security/virtual-servers", nil, "api")
}

// UrlFilter 对应 security/url-filter。
func (s *Security) UrlFilter() (map[string]interface{}, error) {
	return s.S.Get("security/url-filter", nil, "api")
}

// SetUrlFilter 设置 URL 过滤。对应 security/url-filter (post_set)。
// urlfilters 结构与 UrlFilter 返回值相同。
func (s *Security) SetUrlFilter(urlfilters map[string]interface{}) (interface{}, error) {
	return s.S.PostSet("security/url-filter", urlfilters, false, "api", false, false)
}

// Upnp 对应 security/upnp。
func (s *Security) Upnp() (map[string]interface{}, error) {
	return s.S.Get("security/upnp", nil, "api")
}

// SetUpnp 设置 UPnP。对应 security/upnp (post_set)。
func (s *Security) SetUpnp(enabled bool) (interface{}, error) {
	return s.S.PostSet("security/upnp", map[string]interface{}{
		"UpnpStatus": boolToInt(enabled),
	}, false, "api", false, false)
}

// Dmz 对应 security/dmz。
func (s *Security) Dmz() (map[string]interface{}, error) {
	return s.S.Get("security/dmz", nil, "api")
}

// SetDmz 设置 DMZ。对应 security/dmz (post_set)。
func (s *Security) SetDmz(enabled bool, ipAddress string) (interface{}, error) {
	return s.S.PostSet("security/dmz", session.O(
		"DmzStatus", boolToInt(enabled),
		"DmzIPAddress", ipAddress,
	), false, "api", false, false)
}

// Sip 对应 security/sip。
func (s *Security) Sip() (map[string]interface{}, error) {
	return s.S.Get("security/sip", nil, "api")
}

// SetSip 设置 SIP。对应 security/sip (post_set)。
func (s *Security) SetSip(enabled bool, port int) (interface{}, error) {
	return s.S.PostSet("security/sip", session.O(
		"SipStatus", boolToInt(enabled),
		"SipPort", port,
	), false, "api", false, false)
}

// FeatureSwitch 对应 security/feature-switch。
func (s *Security) FeatureSwitch() (map[string]interface{}, error) {
	return s.S.Get("security/feature-switch", nil, "api")
}

// Nat 对应 security/nat。
func (s *Security) Nat() (map[string]interface{}, error) {
	return s.S.Get("security/nat", nil, "api")
}

// SpecialApplications 对应 security/special-applications。
func (s *Security) SpecialApplications() (map[string]interface{}, error) {
	return s.S.Get("security/special-applications", nil, "api")
}

// WhiteLanIpFilter 对应 security/white-lan-ip-filter。
func (s *Security) WhiteLanIpFilter() (map[string]interface{}, error) {
	return s.S.Get("security/white-lan-ip-filter", nil, "api")
}

// WhiteUrlFilter 对应 security/white-url-filter。
func (s *Security) WhiteUrlFilter() (map[string]interface{}, error) {
	return s.S.Get("security/white-url-filter", nil, "api")
}

// SetWhiteUrlFilter 设置白名单 URL 过滤。对应 security/white-url-filter (post_set)。
// urlfilters 结构与 WhiteUrlFilter 返回值相同。
func (s *Security) SetWhiteUrlFilter(urlfilters map[string]interface{}) (interface{}, error) {
	return s.S.PostSet("security/white-url-filter", urlfilters, false, "api", false, false)
}

// Acls 对应 security/acls。
func (s *Security) Acls() (map[string]interface{}, error) {
	return s.S.Get("security/acls", nil, "api")
}

// Acl 对应 security/acl。
func (s *Security) Acl() (map[string]interface{}, error) {
	return s.S.Get("security/acl", nil, "api")
}
