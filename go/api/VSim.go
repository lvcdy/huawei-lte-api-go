package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// VSim 对应 VSim.py。
type VSim struct {
	*session.ApiGroup
}

// NewVSim 创建 VSim API 分组。
func NewVSim(s *session.Session) *VSim {
	return &VSim{ApiGroup: session.NewApiGroup(s)}
}

// OperateswitchVsim 对应 vsim/operateswitch-vsim。
func (v *VSim) OperateswitchVsim() (map[string]interface{}, error) {
	return v.S.Get("vsim/operateswitch-vsim", nil, "api")
}