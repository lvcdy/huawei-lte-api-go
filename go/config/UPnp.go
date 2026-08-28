package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// UPnp 对应 config/UPnp.py。
type UPnp struct {
	*session.ApiGroup
}

// NewUPnp 创建 UPnp API 分组。
func NewUPnp(s *session.Session) *UPnp {
	return &UPnp{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 upnp/config.xml。
func (u *UPnp) Config() (map[string]interface{}, error) {
	return u.S.Get("upnp/config.xml", nil, "config")
}