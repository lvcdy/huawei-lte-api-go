package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Pincode 对应 config/Pincode.py。
type Pincode struct {
	*session.ApiGroup
}

// NewPincode 创建 Pincode API 分组。
func NewPincode(s *session.Session) *Pincode {
	return &Pincode{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 pincode/config.xml。
func (p *Pincode) Config() (map[string]interface{}, error) {
	return p.S.Get("pincode/config.xml", nil, "config")
}
