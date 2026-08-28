package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// FastBoot 对应 config/FastBoot.py。
type FastBoot struct {
	*session.ApiGroup
}

// NewFastBoot 创建 FastBoot API 分组。
func NewFastBoot(s *session.Session) *FastBoot {
	return &FastBoot{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 fastboot/config.xml。
func (f *FastBoot) Config() (map[string]interface{}, error) {
	return f.S.Get("fastboot/config.xml", nil, "config")
}