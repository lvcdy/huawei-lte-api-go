package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Redirection 对应 Redirection.py。
type Redirection struct {
	*session.ApiGroup
}

// NewRedirection 创建 Redirection API 分组。
func NewRedirection(s *session.Session) *Redirection {
	return &Redirection{ApiGroup: session.NewApiGroup(s)}
}

// Homepage 对应 redirection/homepage。
func (r *Redirection) Homepage() (map[string]interface{}, error) {
	return r.S.Get("redirection/homepage", nil, "api")
}