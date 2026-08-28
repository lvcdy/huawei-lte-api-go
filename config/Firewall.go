package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Firewall 对应 config/Firewall.py。
type Firewall struct {
	*session.ApiGroup
}

// NewFirewall 创建 Firewall API 分组。
func NewFirewall(s *session.Session) *Firewall {
	return &Firewall{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 firewall/config.xml。
func (f *Firewall) Config() (map[string]interface{}, error) {
	return f.S.Get("firewall/config.xml", nil, "config")
}
