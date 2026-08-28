package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// MLog 对应 MLog.py。
type MLog struct {
	*session.ApiGroup
}

// NewMLog 创建 MLog API 分组。
func NewMLog(s *session.Session) *MLog {
	return &MLog{ApiGroup: session.NewApiGroup(s)}
}

// MobileLogger 对应 mlog/mobile-logger。
func (m *MLog) MobileLogger() (map[string]interface{}, error) {
	return m.S.Get("mlog/mobile-logger", nil, "api")
}