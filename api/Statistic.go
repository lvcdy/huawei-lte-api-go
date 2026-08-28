package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Statistic 对应 Statistic.py。
type Statistic struct {
	*session.ApiGroup
}

// NewStatistic 创建 Statistic API 分组。
func NewStatistic(s *session.Session) *Statistic {
	return &Statistic{ApiGroup: session.NewApiGroup(s)}
}

// FeatureRoamStatistic 对应 statistic/feature-roam-statistic。
func (s *Statistic) FeatureRoamStatistic() (map[string]interface{}, error) {
	return s.S.Get("statistic/feature-roam-statistic", nil, "api")
}
