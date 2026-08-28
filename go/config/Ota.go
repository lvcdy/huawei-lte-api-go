package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Ota 对应 config/Ota.py。
type Ota struct {
	*session.ApiGroup
}

// NewOta 创建 Ota API 分组。
func NewOta(s *session.Session) *Ota {
	return &Ota{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 ota/config.xml。
func (o *Ota) Config() (map[string]interface{}, error) {
	return o.S.Get("ota/config.xml", nil, "config")
}