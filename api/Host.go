package api

import (
	"time"

	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Host 对应 Host.py。
type Host struct {
	*session.ApiGroup
}

// NewHost 创建 Host API 分组。
func NewHost(s *session.Session) *Host {
	return &Host{ApiGroup: session.NewApiGroup(s)}
}

// Info 上报主机信息。对应 host/info (post_set)。
//
// Python 语义：Platform=platform, PlatformVer=userAgent,
// Navigator=version, NavigatorVer=userAgent。
// dateTime 格式化为 "20060102150405"，时区为 "GMT-0700"。
func (h *Host) Info(dateTime time.Time, platform, userAgent, version string) (interface{}, error) {
	return h.S.PostSet("host/info", session.O(
		"Time", dateTime.Format("20060102150405"),
		"Timezone", "GMT"+dateTime.Format("-0700"),
		"Platform", platform,
		"PlatformVer", userAgent,
		"Navigator", version,
		"NavigatorVer", userAgent,
	), false, "api", false, false)
}
