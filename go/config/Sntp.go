package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Sntp 对应 config/Sntp.py。
type Sntp struct {
	*session.ApiGroup
}

// NewSntp 创建 Sntp API 分组。
func NewSntp(s *session.Session) *Sntp {
	return &Sntp{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 sntp/config.xml。
func (s *Sntp) Config() (map[string]interface{}, error) {
	return s.S.Get("sntp/config.xml", nil, "config")
}