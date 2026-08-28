package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// WebServer 对应 WebServer.py。
type WebServer struct {
	*session.ApiGroup
}

// NewWebServer 创建 WebServer API 分组。
func NewWebServer(s *session.Session) *WebServer {
	return &WebServer{ApiGroup: session.NewApiGroup(s)}
}

// Publickey 对应 webserver/publickey。
func (w *WebServer) Publickey() (map[string]interface{}, error) {
	return w.S.Get("webserver/publickey", nil, "api")
}

// Token 对应 webserver/token。
func (w *WebServer) Token() (map[string]interface{}, error) {
	return w.S.Get("webserver/token", nil, "api")
}

// WhiteListSwitch 对应 webserver/white-list-switch。
func (w *WebServer) WhiteListSwitch() (map[string]interface{}, error) {
	return w.S.Get("webserver/white-list-switch", nil, "api")
}

// SesTokInfo 对应 webserver/SesTokInfo。
func (w *WebServer) SesTokInfo() (map[string]interface{}, error) {
	return w.S.Get("webserver/SesTokInfo", nil, "api")
}
