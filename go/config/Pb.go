package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Pb 对应 config/Pb.py。
type Pb struct {
	*session.ApiGroup
}

// NewPb 创建 Pb API 分组。
func NewPb(s *session.Session) *Pb {
	return &Pb{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 pb/config.xml。
func (p *Pb) Config() (map[string]interface{}, error) {
	return p.S.Get("pb/config.xml", nil, "config")
}