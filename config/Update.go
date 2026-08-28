package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Update 对应 config/Update.py。
type Update struct {
	*session.ApiGroup
}

// NewUpdate 创建 Update API 分组。
func NewUpdate(s *session.Session) *Update {
	return &Update{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 update/config.xml。
func (u *Update) Config() (map[string]interface{}, error) {
	return u.S.Get("update/config.xml", nil, "config")
}
