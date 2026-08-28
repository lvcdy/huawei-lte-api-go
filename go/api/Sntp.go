package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Sntp 对应 SNtp.py。
type Sntp struct {
	*session.ApiGroup
}

// NewSntp 创建 Sntp API 分组。
func NewSntp(s *session.Session) *Sntp {
	return &Sntp{ApiGroup: session.NewApiGroup(s)}
}

// GetSettings 对应 sntp/settings。
func (s *Sntp) GetSettings() (map[string]interface{}, error) {
	return s.S.Get("sntp/settings", nil, "api")
}

// Sntpswitch 对应 sntp/sntpswitch。
func (s *Sntp) Sntpswitch() (map[string]interface{}, error) {
	return s.S.Get("sntp/sntpswitch", nil, "api")
}

// Serverinfo 对应 sntp/serverinfo。
func (s *Sntp) Serverinfo() (map[string]interface{}, error) {
	return s.S.Get("sntp/serverinfo", nil, "api")
}

// Timeinfo 对应 sntp/timeinfo。
func (s *Sntp) Timeinfo() (map[string]interface{}, error) {
	return s.S.Get("sntp/timeinfo", nil, "api")
}