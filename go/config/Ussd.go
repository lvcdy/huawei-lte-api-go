package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Ussd 对应 config/Ussd.py。
type Ussd struct {
	*session.ApiGroup
}

// NewUssd 创建 Ussd API 分组。
func NewUssd(s *session.Session) *Ussd {
	return &Ussd{ApiGroup: session.NewApiGroup(s)}
}

// Prepaidussd 对应 ussd/prepaidussd.xml。
func (u *Ussd) Prepaidussd() (map[string]interface{}, error) {
	return u.S.Get("ussd/prepaidussd.xml", nil, "config")
}

// Postpaidussd 对应 ussd/postpaidussd.xml。
func (u *Ussd) Postpaidussd() (map[string]interface{}, error) {
	return u.S.Get("ussd/postpaidussd.xml", nil, "config")
}