package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Syslog 对应 Syslog.py。
type Syslog struct {
	*session.ApiGroup
}

// NewSyslog 创建 Syslog API 分组。
func NewSyslog(s *session.Session) *Syslog {
	return &Syslog{ApiGroup: session.NewApiGroup(s)}
}

// Querylog 对应 syslog/querylog。
func (s *Syslog) Querylog() (map[string]interface{}, error) {
	return s.S.Get("syslog/querylog", nil, "api")
}

// Clear 清空系统日志。对应 syslog/processlog (post_set, {"command":"clear"})。
func (s *Syslog) Clear() (interface{}, error) {
	return s.S.PostSet("syslog/processlog", map[string]interface{}{
		"command": "clear",
	}, false, "api", false, false)
}