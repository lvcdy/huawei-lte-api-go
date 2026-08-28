package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Statistic 对应 config/Statistic.py。
type Statistic struct {
	*session.ApiGroup
}

// NewStatistic 创建 Statistic API 分组。
func NewStatistic(s *session.Session) *Statistic {
	return &Statistic{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 statistic/config.xml。
func (s *Statistic) Config() (map[string]interface{}, error) {
	return s.S.Get("statistic/config.xml", nil, "config")
}