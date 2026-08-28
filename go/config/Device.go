package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Device 对应 config/Device.py。
type Device struct {
	*session.ApiGroup
}

// NewDevice 创建 Device API 分组。
func NewDevice(s *session.Session) *Device {
	return &Device{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 device/config.xml。
func (d *Device) Config() (map[string]interface{}, error) {
	return d.S.Get("device/config.xml", nil, "config")
}