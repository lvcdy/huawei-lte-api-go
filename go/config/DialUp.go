package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// DialUp 对应 config/DialUp.py。
type DialUp struct {
	*session.ApiGroup
}

// NewDialUp 创建 DialUp API 分组。
func NewDialUp(s *session.Session) *DialUp {
	return &DialUp{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 dialup/config.xml。
func (d *DialUp) Config() (map[string]interface{}, error) {
	return d.S.Get("dialup/config.xml", nil, "config")
}

// Connectmode 对应 dialup/connectmode.xml。
func (d *DialUp) Connectmode() (map[string]interface{}, error) {
	return d.S.Get("dialup/connectmode.xml", nil, "config")
}

// Profileswitch 对应 dialup/profileswitch.xml。
func (d *DialUp) Profileswitch() (map[string]interface{}, error) {
	return d.S.Get("dialup/profileswitch.xml", nil, "config")
}

// LmtAutoModeDisconnect 对应 dialup/lmt_auto_mode_disconnect.xml。
func (d *DialUp) LmtAutoModeDisconnect() (map[string]interface{}, error) {
	return d.S.Get("dialup/lmt_auto_mode_disconnect.xml", nil, "config")
}