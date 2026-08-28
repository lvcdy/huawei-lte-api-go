package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// WebSd 对应 config/WebSd.py。
type WebSd struct {
	*session.ApiGroup
}

// NewWebSd 创建 WebSd API 分组。
func NewWebSd(s *session.Session) *WebSd {
	return &WebSd{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 websd/config.xml。
func (w *WebSd) Config() (map[string]interface{}, error) {
	return w.S.Get("websd/config.xml", nil, "config")
}
