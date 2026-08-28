package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Global 对应 config/Global.py。
type Global struct {
	*session.ApiGroup
}

// NewGlobal 创建 Global API 分组。
func NewGlobal(s *session.Session) *Global {
	return &Global{ApiGroup: session.NewApiGroup(s)}
}

// Languagelist 对应 global/languagelist.xml。
func (g *Global) Languagelist() (map[string]interface{}, error) {
	return g.S.Get("global/languagelist.xml", nil, "config")
}

// Config 对应 global/config.xml。
func (g *Global) Config() (map[string]interface{}, error) {
	return g.S.Get("global/config.xml", nil, "config")
}

// NetType 对应 global/net-type.xml。
func (g *Global) NetType() (map[string]interface{}, error) {
	return g.S.Get("global/net-type.xml", nil, "config")
}
