package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// System 对应 System.py。
type System struct {
	*session.ApiGroup
}

// NewSystem 创建 System API 分组。
func NewSystem(s *session.Session) *System {
	return &System{ApiGroup: session.NewApiGroup(s)}
}

// Devcapacity 对应 system/devcapacity。
func (s *System) Devcapacity() (map[string]interface{}, error) {
	return s.S.Get("system/devcapacity", nil, "api")
}

// Deviceinfo 对应 system/deviceinfo。
func (s *System) Deviceinfo() (map[string]interface{}, error) {
	return s.S.Get("system/deviceinfo", nil, "api")
}

// Deviceinfoex 对应 system/deviceinfoex。
func (s *System) Deviceinfoex() (map[string]interface{}, error) {
	return s.S.Get("system/deviceinfoex", nil, "api")
}

// Onlineupg 检查在线升级。对应 system/onlineupg (post_get, is_json)。
func (s *System) Onlineupg() (map[string]interface{}, error) {
	return s.S.PostGet("system/onlineupg", map[string]interface{}{
		"action": "check",
		"data": map[string]interface{}{
			"UpdateAction": 1,
		},
	}, false, "api", false, true)
}

// Onlinestate 获取在线状态。对应 system/onlinestate。
// Python 返回 list；Go 中返回的 map 需调用方自行转 []interface{}。
func (s *System) Onlinestate(devid string) (map[string]interface{}, error) {
	return s.S.Get("system/onlinestate", map[string]string{"devid": devid}, "api")
}

// Hostinfo 对应 system/HostInfo。
// Python 返回 list；Go 中返回的 map 需调用方自行转 []interface{}。
func (s *System) Hostinfo() (map[string]interface{}, error) {
	return s.S.Get("system/HostInfo", nil, "api")
}