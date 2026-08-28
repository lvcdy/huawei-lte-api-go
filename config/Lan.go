package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Lan 对应 config/Lan.py。
type Lan struct {
	*session.ApiGroup
}

// NewLan 创建 Lan API 分组。
func NewLan(s *session.Session) *Lan {
	return &Lan{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 lan/config.xml。
func (l *Lan) Config() (map[string]interface{}, error) {
	return l.S.Get("lan/config.xml", nil, "config")
}
