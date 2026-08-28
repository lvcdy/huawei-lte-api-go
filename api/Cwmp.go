package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Cwmp 对应 Cwmp.py。
type Cwmp struct {
	*session.ApiGroup
}

// NewCwmp 创建 Cwmp API 分组。
func NewCwmp(s *session.Session) *Cwmp {
	return &Cwmp{ApiGroup: session.NewApiGroup(s)}
}

// BasicInfo 对应 cwmp/basic-info。
func (c *Cwmp) BasicInfo() (map[string]interface{}, error) {
	return c.S.Get("cwmp/basic-info", nil, "api")
}
