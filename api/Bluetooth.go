package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Bluetooth 对应 Bluetooth.py。
type Bluetooth struct {
	*session.ApiGroup
}

// NewBluetooth 创建 Bluetooth API 分组。
func NewBluetooth(s *session.Session) *Bluetooth {
	return &Bluetooth{ApiGroup: session.NewApiGroup(s)}
}

// Settings 对应 bluetooth/settings。
func (b *Bluetooth) Settings() (map[string]interface{}, error) {
	return b.S.Get("bluetooth/settings", nil, "api")
}

// Scan 对应 bluetooth/scan。
func (b *Bluetooth) Scan() (map[string]interface{}, error) {
	return b.S.Get("bluetooth/scan", nil, "api")
}
