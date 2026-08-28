package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// UsbStorage 对应 UsbStorage.py。
type UsbStorage struct {
	*session.ApiGroup
}

// NewUsbStorage 创建 UsbStorage API 分组。
func NewUsbStorage(s *session.Session) *UsbStorage {
	return &UsbStorage{ApiGroup: session.NewApiGroup(s)}
}

// Fsstatus 对应 usbstorage/fsstatus。
func (u *UsbStorage) Fsstatus() (map[string]interface{}, error) {
	return u.S.Get("usbstorage/fsstatus", nil, "api")
}

// Usbaccount 对应 usbstorage/usbaccount。
func (u *UsbStorage) Usbaccount() (map[string]interface{}, error) {
	return u.S.Get("usbstorage/usbaccount", nil, "api")
}
