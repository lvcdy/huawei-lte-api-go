package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Staticroute 对应 Staticroute.py。
type Staticroute struct {
	*session.ApiGroup
}

// NewStaticroute 创建 Staticroute API 分组。
func NewStaticroute(s *session.Session) *Staticroute {
	return &Staticroute{ApiGroup: session.NewApiGroup(s)}
}

// Wanpath 对应 staticroute/wanpath。
func (s *Staticroute) Wanpath() (map[string]interface{}, error) {
	return s.S.Get("staticroute/wanpath", nil, "api")
}
