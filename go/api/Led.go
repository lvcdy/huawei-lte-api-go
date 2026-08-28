package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Led 对应 Led.py。
type Led struct {
	*session.ApiGroup
}

// NewLed 创建 Led API 分组。
func NewLed(s *session.Session) *Led {
	return &Led{ApiGroup: session.NewApiGroup(s)}
}

// Nightmode 对应 led/nightmode。
func (l *Led) Nightmode() (map[string]interface{}, error) {
	return l.S.Get("led/nightmode", nil, "api")
}

// Appctrlled 对应 led/appctrlled。
func (l *Led) Appctrlled() (map[string]interface{}, error) {
	return l.S.Get("led/appctrlled", nil, "api")
}