package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Ussd 对应 Ussd.py。
type Ussd struct {
	*session.ApiGroup
}

// NewUssd 创建 Ussd API 分组。
func NewUssd(s *session.Session) *Ussd {
	return &Ussd{ApiGroup: session.NewApiGroup(s)}
}

// Status 对应 ussd/status。
func (u *Ussd) Status() (map[string]interface{}, error) {
	return u.S.Get("ussd/status", nil, "api")
}

// Get 对应 ussd/get。
func (u *Ussd) Get() (map[string]interface{}, error) {
	return u.S.Get("ussd/get", nil, "api")
}

// Send 发送 USSD 指令。对应 ussd/send (post_get)。
func (u *Ussd) Send(content string) (map[string]interface{}, error) {
	return u.S.PostGet("ussd/send", map[string]interface{}{
		"content":  content,
		"codeType": "codeType",
		"timeout":  nil,
	}, false, "api", false, false)
}
