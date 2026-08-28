package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Time 对应 Time.py。
type Time struct {
	*session.ApiGroup
}

// NewTime 创建 Time API 分组。
func NewTime(s *session.Session) *Time {
	return &Time{ApiGroup: session.NewApiGroup(s)}
}

// Timeout 对应 time/timeout。
func (t *Time) Timeout() (map[string]interface{}, error) {
	return t.S.Get("time/timeout", nil, "api")
}
