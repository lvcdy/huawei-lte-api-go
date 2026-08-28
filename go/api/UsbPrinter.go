package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// UsbPrinter 对应 UsbPrinter.py。
type UsbPrinter struct {
	*session.ApiGroup
}

// NewUsbPrinter 创建 UsbPrinter API 分组。
func NewUsbPrinter(s *session.Session) *UsbPrinter {
	return &UsbPrinter{ApiGroup: session.NewApiGroup(s)}
}

// Printerlist 对应 usbprinter/printerlist。
func (u *UsbPrinter) Printerlist() (map[string]interface{}, error) {
	return u.S.Get("usbprinter/printerlist", nil, "api")
}