package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// TimeRule 对应 TimeRule.py。
type TimeRule struct {
	*session.ApiGroup
}

// NewTimeRule 创建 TimeRule API 分组。
func NewTimeRule(s *session.Session) *TimeRule {
	return &TimeRule{ApiGroup: session.NewApiGroup(s)}
}

// Timerule 对应 timerule/timerule。
func (t *TimeRule) Timerule() (map[string]interface{}, error) {
	return t.S.Get("timerule/timerule", nil, "api")
}
