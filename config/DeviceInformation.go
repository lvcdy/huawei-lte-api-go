package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// DeviceInformation 对应 config/DeviceInformation.py。
type DeviceInformation struct {
	*session.ApiGroup
}

// NewDeviceInformation 创建 DeviceInformation API 分组。
func NewDeviceInformation(s *session.Session) *DeviceInformation {
	return &DeviceInformation{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 deviceinformation/config.xml。
func (d *DeviceInformation) Config() (map[string]interface{}, error) {
	return d.S.Get("deviceinformation/config.xml", nil, "config")
}
