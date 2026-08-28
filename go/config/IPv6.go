package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// IPv6 对应 config/IPv6.py。
type IPv6 struct {
	*session.ApiGroup
}

// NewIPv6 创建 IPv6 API 分组。
func NewIPv6(s *session.Session) *IPv6 {
	return &IPv6{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 ipv6/config.xml。
func (i *IPv6) Config() (map[string]interface{}, error) {
	return i.S.Get("ipv6/config.xml", nil, "config")
}