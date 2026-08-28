package usermanual

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// PublicSysResources 对应 usermanual/PublicSysResources.py。
type PublicSysResources struct {
	*session.ApiGroup
}

// NewPublicSysResources 创建 PublicSysResources API 分组。
func NewPublicSysResources(s *session.Session) *PublicSysResources {
	return &PublicSysResources{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 public_sys-resources/config.xml。
func (p *PublicSysResources) Config() (map[string]interface{}, error) {
	return p.S.Get("public_sys-resources/config.xml", nil, "usermanual")
}