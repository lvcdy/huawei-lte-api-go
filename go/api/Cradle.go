package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Cradle 对应 Cradle.py。
type Cradle struct {
	*session.ApiGroup
}

// NewCradle 创建 Cradle API 分组。
func NewCradle(s *session.Session) *Cradle {
	return &Cradle{ApiGroup: session.NewApiGroup(s)}
}

// StatusInfo 对应 cradle/status-info。
func (c *Cradle) StatusInfo() (map[string]interface{}, error) {
	return c.S.Get("cradle/status-info", nil, "api")
}

// FeatureSwitch 对应 cradle/feature-switch。
func (c *Cradle) FeatureSwitch() (map[string]interface{}, error) {
	return c.S.Get("cradle/feature-switch", nil, "api")
}

// BasicInfo 对应 cradle/basic-info。
func (c *Cradle) BasicInfo() (map[string]interface{}, error) {
	return c.S.Get("cradle/basic-info", nil, "api")
}

// FactoryMac 对应 cradle/factory-mac。
func (c *Cradle) FactoryMac() (map[string]interface{}, error) {
	return c.S.Get("cradle/factory-mac", nil, "api")
}

// MacInfo 对应 cradle/mac-info。
func (c *Cradle) MacInfo() (map[string]interface{}, error) {
	return c.S.Get("cradle/mac-info", nil, "api")
}