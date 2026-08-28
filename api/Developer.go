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

// DevelopermodeFeatureswitch 对应 developer/developermode-featureswitch。
func (d *Developer) DevelopermodeFeatureswitch() (map[string]interface{}, error) {
	return d.S.Get("developer/developermode-featureswitch", nil, "api")
}

// AtportStatus 对应 app/atport-status。
func (d *Developer) AtportStatus() (map[string]interface{}, error) {
	return d.S.Get("app/atport-status", nil, "api")
}
