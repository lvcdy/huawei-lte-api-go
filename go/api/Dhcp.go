package api

import (
	"fmt"
	"strings"

	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Dhcp 对应 Dhcp.py。
type Dhcp struct {
	*session.ApiGroup
}

// NewDhcp 创建 Dhcp API 分组。
func NewDhcp(s *session.Session) *Dhcp {
	return &Dhcp{ApiGroup: session.NewApiGroup(s)}
}

// Settings 对应 dhcp/settings。
func (d *Dhcp) Settings() (map[string]interface{}, error) {
	return d.S.Get("dhcp/settings", nil, "api")
}

// FeatureSwitch 对应 dhcp/feature-switch。
func (d *Dhcp) FeatureSwitch() (map[string]interface{}, error) {
	return d.S.Get("dhcp/feature-switch", nil, "api")
}

// DhcpHostInfo 对应 dhcp/dhcp-host-info。
func (d *Dhcp) DhcpHostInfo() (map[string]interface{}, error) {
	return d.S.Get("dhcp/dhcp-host-info", nil, "api")
}

// StaticAddrInfo 对应 dhcp/static-addr-info。
func (d *Dhcp) StaticAddrInfo() (map[string]interface{}, error) {
	return d.S.Get("dhcp/static-addr-info", nil, "api")
}

// SetSettings 配置 DHCP 服务器。对应 dhcp/settings (post_set)。
func (d *Dhcp) SetSettings(
	dhcpIPAddress string,
	dhcpLanNetmask string,
	dhcpStatus bool,
	dhcpStartIPRange int,
	dhcpEndIPRange int,
	dhcpLeaseTime int,
	dnsStatus bool,
	primaryDNS *string,
	secondaryDNS *string,
	showDNSSetting bool,
) (interface{}, error) {
	ipParts := strings.Split(dhcpIPAddress, ".")
	ipParts = ipParts[:len(ipParts)-1]
	prefix := strings.Join(ipParts, ".")
	startAddr := fmt.Sprintf("%s.%d", prefix, dhcpStartIPRange)
	endAddr := fmt.Sprintf("%s.%d", prefix, dhcpEndIPRange)

	data := map[string]interface{}{
		"DhcpIPAddress":      dhcpIPAddress,
		"DhcpLanNetmask":     dhcpLanNetmask,
		"DhcpStatus":         boolToInt(dhcpStatus),
		"DhcpStartIPAddress": startAddr,
		"DhcpEndIPAddress":   endAddr,
		"DhcpLeaseTime":      dhcpLeaseTime,
		"DnsStatus":          boolToInt(dnsStatus),
		"PrimaryDns":         nil,
		"SecondaryDns":       nil,
		"ShowDnsSetting":     boolToInt(showDNSSetting),
	}
	if primaryDNS != nil {
		data["PrimaryDns"] = *primaryDNS
	}
	if secondaryDNS != nil {
		data["SecondaryDns"] = *secondaryDNS
	}
	return d.S.PostSet("dhcp/settings", data, false, "api", false, false)
}