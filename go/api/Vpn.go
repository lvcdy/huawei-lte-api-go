package api

import (
	"github.com/lvcdy/huawei-lte-api-go/enums"
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Vpn 对应 Vpn.py。
type Vpn struct {
	*session.ApiGroup
}

// NewVpn 创建 Vpn API 分组。
func NewVpn(s *session.Session) *Vpn {
	return &Vpn{ApiGroup: session.NewApiGroup(s)}
}

// FeatureSwitch 对应 vpn/feature-switch。
func (v *Vpn) FeatureSwitch() (map[string]interface{}, error) {
	return v.S.Get("vpn/feature-switch", nil, "api")
}

// BrList 对应 vpn/br_list。
func (v *Vpn) BrList() (map[string]interface{}, error) {
	return v.S.Get("vpn/br_list", nil, "api")
}

// IpsecSettings 对应 vpn/ipsec_settings。
func (v *Vpn) IpsecSettings() (map[string]interface{}, error) {
	return v.S.Get("vpn/ipsec_settings", nil, "api")
}

// L2tpSettings 对应 vpn/l2tp_settings。
func (v *Vpn) L2tpSettings() (map[string]interface{}, error) {
	return v.S.Get("vpn/l2tp_settings", nil, "api")
}

// PptpSettings 对应 vpn/pptp_settings。
func (v *Vpn) PptpSettings() (map[string]interface{}, error) {
	return v.S.Get("vpn/pptp_settings", nil, "api")
}

// ToggleStatus 启用/禁用指定类型的 VPN。
// 对应 vpn/{pptp|l2tp}_settings (post_set, is_encrypted)。
func (v *Vpn) ToggleStatus(enable bool, vpnType enums.VPNType) (interface{}, error) {
	enableStr := "0"
	if enable {
		enableStr = "1"
	}
	return v.S.PostSet("vpn/"+string(vpnType)+"_settings", map[string]interface{}{
		"enable": enableStr,
	}, false, "api", true, false)
}

// Status 对应 vpn/status。
func (v *Vpn) Status() (map[string]interface{}, error) {
	return v.S.Get("vpn/status", nil, "api")
}