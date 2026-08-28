package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Network 对应 config/Network.py。
type Network struct {
	*session.ApiGroup
}

// NewNetwork 创建 Network API 分组。
func NewNetwork(s *session.Session) *Network {
	return &Network{ApiGroup: session.NewApiGroup(s)}
}

// NetMode 对应 network/net-mode.xml。
func (n *Network) NetMode() (map[string]interface{}, error) {
	return n.S.Get("network/net-mode.xml", nil, "config")
}

// Networkmode 对应 network/networkmode.xml。
func (n *Network) Networkmode() (map[string]interface{}, error) {
	return n.S.Get("network/networkmode.xml", nil, "config")
}

// Config 对应 network/config.xml。
func (n *Network) Config() (map[string]interface{}, error) {
	return n.S.Get("network/config.xml", nil, "config")
}

// NetworkbandNull 对应 network/networkband_null.xml。
func (n *Network) NetworkbandNull() (map[string]interface{}, error) {
	return n.S.Get("network/networkband_null.xml", nil, "config")
}

// SetOnly4g 对应 network/setOnly4g.xml。
func (n *Network) SetOnly4g() (map[string]interface{}, error) {
	return n.S.Get("network/setOnly4g.xml", nil, "config")
}