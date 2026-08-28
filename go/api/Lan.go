package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Lan 对应 Lan.py。
type Lan struct {
	*session.ApiGroup
}

// NewLan 创建 Lan API 分组。
func NewLan(s *session.Session) *Lan {
	return &Lan{ApiGroup: session.NewApiGroup(s)}
}

// HostInfo 获取主机信息。对应 lan/HostInfo (get) + enforce_list_response("Host")。
func (l *Lan) HostInfo() (map[string]interface{}, error) {
	res, err := l.S.Get("lan/HostInfo", nil, "api")
	if err != nil {
		return nil, err
	}
	return session.EnforceListResponse(res, "Host", nil), nil
}