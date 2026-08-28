package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Stk 对应 config/Stk.py。
type Stk struct {
	*session.ApiGroup
}

// NewStk 创建 Stk API 分组。
func NewStk(s *session.Session) *Stk {
	return &Stk{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 stk/config.xml。
func (s *Stk) Config() (map[string]interface{}, error) {
	return s.S.Get("stk/config.xml", nil, "config")
}