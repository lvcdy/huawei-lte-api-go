package config

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// PcAssistant 对应 config/PcAssistant.py。
type PcAssistant struct {
	*session.ApiGroup
}

// NewPcAssistant 创建 PcAssistant API 分组。
func NewPcAssistant(s *session.Session) *PcAssistant {
	return &PcAssistant{ApiGroup: session.NewApiGroup(s)}
}

// Config 对应 pcassistant/config.xml。
func (p *PcAssistant) Config() (map[string]interface{}, error) {
	return p.S.Get("pcassistant/config.xml", nil, "config")
}

// Updateautorun 对应 pcassistant/updateautorun.xml。
func (p *PcAssistant) Updateautorun() (map[string]interface{}, error) {
	return p.S.Get("pcassistant/updateautorun.xml", nil, "config")
}
