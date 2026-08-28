package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Global 对应 Global.py。
type Global struct {
	*session.ApiGroup
}

// NewGlobal 创建 Global API 分组。
func NewGlobal(s *session.Session) *Global {
	return &Global{ApiGroup: session.NewApiGroup(s)}
}

// ModuleSwitch 对应 global/module-switch。
func (g *Global) ModuleSwitch() (map[string]interface{}, error) {
	return g.S.Get("global/module-switch", nil, "api")
}

// StorageGetItem 对应 global/storage-getitem。
func (g *Global) StorageGetItem() (map[string]interface{}, error) {
	return g.S.Get("global/storage-getitem", nil, "api")
}

// CustommenuUrl 对应 global/custommenu-url。
func (g *Global) CustommenuUrl() (map[string]interface{}, error) {
	return g.S.Get("global/custommenu-url", nil, "api")
}
