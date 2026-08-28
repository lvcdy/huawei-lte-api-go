package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Voice 对应 config/Voice.py。
type Voice struct {
	*session.ApiGroup
}

// NewVoice 创建 Voice API 分组。
func NewVoice(s *session.Session) *Voice {
	return &Voice{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 voice/config.xml。
func (v *Voice) Config() (map[string]interface{}, error) {
	return v.S.Get("voice/config.xml", nil, "config")
}

// Country 对应 voice/country.xml。
func (v *Voice) Country() (map[string]interface{}, error) {
	return v.S.Get("voice/country.xml", nil, "config")
}