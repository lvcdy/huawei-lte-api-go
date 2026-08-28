package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Wifi 对应 config/Wifi.py。
type Wifi struct {
	*session.ApiGroup
}

// NewWifi 创建 Wifi API 分组。
func NewWifi(s *session.Session) *Wifi {
	return &Wifi{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 wifi/config.xml。
func (w *Wifi) Config() (map[string]interface{}, error) {
	return w.S.Get("wifi/config.xml", nil, "config")
}

// Configure 对应 wifi/configure.xml。
func (w *Wifi) Configure() (map[string]interface{}, error) {
	return w.S.Get("wifi/configure.xml", nil, "config")
}

// CountryChannel 对应 wifi/countryChannel.xml。
func (w *Wifi) CountryChannel() (map[string]interface{}, error) {
	return w.S.Get("wifi/countryChannel.xml", nil, "config")
}

// ChannelAutoMatchHardware 对应 wifi/channelAutoMatchHardware.xml。
func (w *Wifi) ChannelAutoMatchHardware() (map[string]interface{}, error) {
	return w.S.Get("wifi/channelAutoMatchHardware.xml", nil, "config")
}