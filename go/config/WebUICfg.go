package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// WebUICfg 对应 config/WebUICfg.py。
type WebUICfg struct {
	*session.ApiGroup
}

// NewWebUICfg 创建 WebUICfg API 分组。
func NewWebUICfg(s *session.Session) *WebUICfg {
	return &WebUICfg{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 webuicfg/config.xml。
func (w *WebUICfg) Config() (map[string]interface{}, error) {
	return w.S.Get("webuicfg/config.xml", nil, "config")
}