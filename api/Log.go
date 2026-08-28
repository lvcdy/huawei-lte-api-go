package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Log 对应 Log.py。
type Log struct {
	*session.ApiGroup
}

// NewLog 创建 Log API 分组。
func NewLog(s *session.Session) *Log {
	return &Log{ApiGroup: session.NewApiGroup(s)}
}

// Loginfo 对应 log/loginfo。
func (l *Log) Loginfo() (map[string]interface{}, error) {
	return l.S.Get("log/loginfo", nil, "api")
}
