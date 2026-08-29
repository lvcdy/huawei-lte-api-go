package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Developer 对应 Developer.py。
type Developer struct {
	*session.ApiGroup
}

// NewDeveloper 创建 Developer API 分组。
func NewDeveloper(s *session.Session) *Developer {
	return &Developer{ApiGroup: session.NewApiGroup(s)}
}

// DevelopedBy 需要设备开发者登录状态（loginflag=2）。以下端点均在开发者模式下可用。

// DevelopermodeFeatureswitch 对应 developer/developermode-featureswitch。
func (d *Developer) DevelopermodeFeatureswitch() (map[string]interface{}, error) {
	return d.S.Get("developer/developermode-featureswitch", nil, "api")
}

// DeveloperMode 获取开发者模式开关状态。对应 developermode/developer-mode。
// 返回 developer_mode 等字段；部分固件用 app/atport-status 替代。
func (d *Developer) DeveloperMode() (map[string]interface{}, error) {
	return d.S.Get("developermode/developer-mode", nil, "api")
}

// DeveloperItem 获取开发者模式各项开关（telnet/AT 等）。对应 developermode/developer-item。
func (d *Developer) DeveloperItem() (map[string]interface{}, error) {
	return d.S.Get("developermode/developer-item", nil, "api")
}

// AtportStatus 查询 AT 端口状态。对应 app/atport-status。
func (d *Developer) AtportStatus() (map[string]interface{}, error) {
	return d.S.Get("app/atport-status", nil, "api")
}

// SetAtportStatus 启用/关闭 AT 调试端口（如 Telnet 20249）。对应 app/atport-status (post_set)。
//
// 注意：写入该端点需要开发者模式登录（loginflag=2 挑战认证）。
// enable 传 1 开启 telnet 调试端口，0 关闭。
func (d *Developer) SetAtportStatus(enable int) (interface{}, error) {
	return d.S.PostSet("app/atport-status", session.O(
		"enable", enable,
	), false, "api", false, false)
}
