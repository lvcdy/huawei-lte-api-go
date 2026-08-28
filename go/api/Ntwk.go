package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Ntwk 对应 Ntwk.py。
type Ntwk struct {
	*session.ApiGroup
}

// NewNtwk 创建 Ntwk API 分组。
func NewNtwk(s *session.Session) *Ntwk {
	return &Ntwk{ApiGroup: session.NewApiGroup(s)}
}

// LanUpnpPortmapping 对应 ntwk/lan_upnp_portmapping。
func (n *Ntwk) LanUpnpPortmapping() (map[string]interface{}, error) {
	return n.S.Get("ntwk/lan_upnp_portmapping", nil, "api")
}

// Celllock 对应 ntwk/celllock。
func (n *Ntwk) Celllock() (map[string]interface{}, error) {
	return n.S.Get("ntwk/celllock", nil, "api")
}

// Dualwaninfo 对应 ntwk/dualwaninfo。
func (n *Ntwk) Dualwaninfo() (map[string]interface{}, error) {
	return n.S.Get("ntwk/dualwaninfo", nil, "api")
}

// LanWanConfig 对应 ntwk/lan-wan-config。
func (n *Ntwk) LanWanConfig() (map[string]interface{}, error) {
	return n.S.Get("ntwk/lan-wan-config", nil, "api")
}