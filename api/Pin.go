package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Pin 对应 Pin.py。
type Pin struct {
	*session.ApiGroup
}

// NewPin 创建 Pin API 分组。
func NewPin(s *session.Session) *Pin {
	return &Pin{ApiGroup: session.NewApiGroup(s)}
}

// Status 对应 pin/status。
func (p *Pin) Status() (map[string]interface{}, error) {
	return p.S.Get("pin/status", nil, "api")
}

// Simlock 对应 pin/simlock。
func (p *Pin) Simlock() (map[string]interface{}, error) {
	return p.S.Get("pin/simlock", nil, "api")
}

// SavePin 对应 pin/save-pin。
func (p *Pin) SavePin() (map[string]interface{}, error) {
	return p.S.Get("pin/save-pin", nil, "api")
}

// Operate 执行 PIN 操作。对应 pin/operate (post_set, is_encrypted)。
//
// operate_type: 0 - 验证 PIN, 1 - 启用 PIN 验证, 2 - 禁用 PIN 验证,
// 3 - 设置新 PIN, 4 - 使用 PUK 码。
func (p *Pin) Operate(operateType string, currentPin *string, newPin *string, pukCode *string) (interface{}, error) {
	data := map[string]interface{}{
		"OperateType": operateType,
		"CurrentPin":  nil,
		"NewPin":      nil,
		"PukCode":     nil,
	}
	if currentPin != nil {
		data["CurrentPin"] = *currentPin
	}
	if newPin != nil {
		data["NewPin"] = *newPin
	}
	if pukCode != nil {
		data["PukCode"] = *pukCode
	}
	return p.S.PostSet("pin/operate", data, false, "api", true, false)
}
