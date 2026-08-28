package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// OnlineUpdate 对应 OnlineUpdate.py。
type OnlineUpdate struct {
	*session.ApiGroup
}

// NewOnlineUpdate 创建 OnlineUpdate API 分组。
func NewOnlineUpdate(s *session.Session) *OnlineUpdate {
	return &OnlineUpdate{ApiGroup: session.NewApiGroup(s)}
}

// CheckNewVersion 对应 online-update/check-new-version。
func (o *OnlineUpdate) CheckNewVersion() (map[string]interface{}, error) {
	return o.S.Get("online-update/check-new-version", nil, "api")
}

// SetCheckNewVersion 对应 online-update/check-new-version (post_set, 无数据)。
func (o *OnlineUpdate) SetCheckNewVersion() (interface{}, error) {
	return o.S.PostSet("online-update/check-new-version", nil, false, "api", false, false)
}

// Status 对应 online-update/status。
func (o *OnlineUpdate) Status() (map[string]interface{}, error) {
	return o.S.Get("online-update/status", nil, "api")
}

// UrlList 对应 online-update/url-list。
func (o *OnlineUpdate) UrlList() (map[string]interface{}, error) {
	return o.S.Get("online-update/url-list", nil, "api")
}

// AckNewversion 对应 online-update/ack-newversion。
func (o *OnlineUpdate) AckNewversion() (map[string]interface{}, error) {
	return o.S.Get("online-update/ack-newversion", nil, "api")
}

// SetAckNewversion 对应 online-update/ack-newversion (post_set)。
func (o *OnlineUpdate) SetAckNewversion() (interface{}, error) {
	return o.S.PostSet("online-update/ack-newversion", map[string]interface{}{
		"userAckNewVersion": 0,
	}, false, "api", false, false)
}

// CancelDownloading 对应 online-update/cancel-downloading。
func (o *OnlineUpdate) CancelDownloading() (map[string]interface{}, error) {
	return o.S.Get("online-update/cancel-downloading", nil, "api")
}

// SetCancelDownloading 对应 online-update/cancel-downloading (post_set, 无数据)。
func (o *OnlineUpdate) SetCancelDownloading() (interface{}, error) {
	return o.S.PostSet("online-update/cancel-downloading", nil, false, "api", false, false)
}

// UpgradeMessagebox 对应 online-update/upgrade-messagebox。
func (o *OnlineUpdate) UpgradeMessagebox() (map[string]interface{}, error) {
	return o.S.Get("online-update/upgrade-messagebox", nil, "api")
}

// SetUpgradeMessagebox 对应 online-update/upgrade-messagebox (post_set)。
func (o *OnlineUpdate) SetUpgradeMessagebox(messagebox string) (interface{}, error) {
	return o.S.PostSet("online-update/upgrade-messagebox", map[string]interface{}{
		"messagebox": messagebox,
	}, false, "api", false, false)
}

// Configuration 对应 online-update/configuration。
func (o *OnlineUpdate) Configuration() (map[string]interface{}, error) {
	return o.S.Get("online-update/configuration", nil, "api")
}

// AutoupdateConfig 对应 online-update/autoupdate-config。
func (o *OnlineUpdate) AutoupdateConfig() (map[string]interface{}, error) {
	return o.S.Get("online-update/autoupdate-config", nil, "api")
}

// SetAutoupdateConfig 对应 online-update/autoupdate-config (post_set)。
func (o *OnlineUpdate) SetAutoupdateConfig(autoupdate bool) (interface{}, error) {
	val := 0
	if autoupdate {
		val = 1
	}
	return o.S.PostSet("online-update/autoupdate-config", map[string]interface{}{
		"auto_update": val,
		"ui_download": 0,
	}, false, "api", false, false)
}

// RedirectCancel 对应 online-update/redirect_cancel。
func (o *OnlineUpdate) RedirectCancel() (map[string]interface{}, error) {
	return o.S.Get("online-update/redirect_cancel", nil, "api")
}
