package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Ota 对应 Ota.py。
type Ota struct {
	*session.ApiGroup
}

// NewOta 创建 Ota API 分组。
func NewOta(s *session.Session) *Ota {
	return &Ota{ApiGroup: session.NewApiGroup(s)}
}

// Status 对应 ota/status。
func (o *Ota) Status() (map[string]interface{}, error) {
	return o.S.Get("ota/status", nil, "api")
}