package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Sms 对应 config/Sms.py。
type Sms struct {
	*session.ApiGroup
}

// NewSms 创建 Sms API 分组。
func NewSms(s *session.Session) *Sms {
	return &Sms{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 sms/config.xml。
func (s *Sms) Config() (map[string]interface{}, error) {
	return s.S.Get("sms/config.xml", nil, "config")
}
