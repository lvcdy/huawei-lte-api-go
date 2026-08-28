// Package api 对应 Python 版 huawei_lte_api.api 的全部 API 分组。
package api

import (
	"fmt"

	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Net 对应 Net.py。
type Net struct {
	*session.ApiGroup
}

// NewNet 创建 Net API 分组。
func NewNet(s *session.Session) *Net {
	return &Net{ApiGroup: session.NewApiGroup(s)}
}

// CurrentPlmn 获取当前 PLMN。对应 net/current-plmn。
func (n *Net) CurrentPlmn() (map[string]interface{}, error) {
	return n.S.Get("net/current-plmn", nil, "api")
}

// NetMode 获取当前网络模式。对应 net/net-mode。
func (n *Net) NetMode() (map[string]interface{}, error) {
	return n.S.Get("net/net-mode", nil, "api")
}

// SetNetMode 设置网络模式。对应 net/net-mode (post_set)。
//
// lteband/networkband 为 int 时自动转小写十六进制（如 0x80005 → "80005"）；
// 为 string 时原样使用。networkmode 为 string 时原样使用。
func (n *Net) SetNetMode(lteband interface{}, networkband interface{}, networkmode interface{}) (interface{}, error) {
	return n.S.PostSet("net/net-mode", session.O(
		"NetworkMode", networkModeValue(networkmode),
		"NetworkBand", bandHexValue(networkband),
		"LTEBand", bandHexValue(lteband),
	), false, "api", false, false)
}

// networkModeValue 返回 networkmode 的字符串值。
// NetworkMode 为 string 型枚举，其底层是 string，经 fmt.Sprintf("%v") 即可恢复。
func networkModeValue(v interface{}) string {
	if s, ok := v.(string); ok {
		return s
	}
	return fmt.Sprintf("%v", v)
}

// bandHexValue 返回 band 的小写十六进制字符串（无 0x 前缀）。
// string 原样返回。
func bandHexValue(v interface{}) string {
	if s, ok := v.(string); ok {
		return s
	}
	return fmt.Sprintf("%x", v)
}

// Network 获取网络信息。对应 net/network。
func (n *Net) Network() (map[string]interface{}, error) {
	return n.S.Get("net/network", nil, "api")
}

// SetNetwork 设置网络模式与频段。对应 net/network (post_set)。
func (n *Net) SetNetwork(networkmode string, networkband string) (interface{}, error) {
	return n.S.PostSet("net/network", session.O(
		"NetworkMode", networkmode,
		"NetworkBand", networkband,
	), false, "api", false, false)
}

// Register 获取网络注册状态。对应 net/register。
func (n *Net) Register() (map[string]interface{}, error) {
	return n.S.Get("net/register", nil, "api")
}

// SetRegister 设置网络注册模式。对应 net/register (post_set)。
func (n *Net) SetRegister(mode string, plmn string, rat string) (interface{}, error) {
	return n.S.PostSet("net/register", session.O(
		"Mode", mode,
		"Plmn", plmn,
		"Rat", rat,
	), false, "api", false, false)
}

// NetModeList 获取可用网络模式列表。对应 net/net-mode-list。
func (n *Net) NetModeList() (map[string]interface{}, error) {
	return n.S.Get("net/net-mode-list", nil, "api")
}

// PlmnList 获取可用 PLMN 列表。对应 net/plmn-list。
func (n *Net) PlmnList() (map[string]interface{}, error) {
	return n.S.Get("net/plmn-list", nil, "api")
}

// NetFeatureSwitch 获取网络功能开关状态。对应 net/net-feature-switch。
func (n *Net) NetFeatureSwitch() (map[string]interface{}, error) {
	return n.S.Get("net/net-feature-switch", nil, "api")
}

// CellInfo 获取小区信息。对应 net/cell-info。
func (n *Net) CellInfo() (map[string]interface{}, error) {
	return n.S.Get("net/cell-info", nil, "api")
}

// CspsState 获取 CSPS 状态。对应 net/csps_state。
func (n *Net) CspsState() (map[string]interface{}, error) {
	return n.S.Get("net/csps_state", nil, "api")
}

// Reconnect 重新连接网络。对应 net/reconnect (post_set)。
func (n *Net) Reconnect() (interface{}, error) {
	return n.S.PostSet("net/reconnect", session.O("ReconnectAction", 1), false, "api", false, false)
}
