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

// LockCell 锁定/解锁指定频点上的小区。对应 net/lock-cell (post_set)。
//
// 5G CPE 锁频管理端点：
//   - lock: 1 锁定 / 0 清除锁频
//   - freq: 目标频点（ARFCN），清除锁频时传 0
//   - pci:  目标小区 PCI，清除锁频时传 0
//
// 与 ntwk/celllock（读取当前锁频参数）配套使用。
func (n *Ntwk) LockCell(lock int, freq int, pci int) (interface{}, error) {
	return n.S.PostSet("net/lock-cell", session.O(
		"LockCell", lock,
		"Freq", freq,
		"PCI", pci,
	), false, "api", false, false)
}
