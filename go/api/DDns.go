package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Ddns 对应 DDns.py。
type Ddns struct {
	*session.ApiGroup
}

// NewDdns 创建 DDns API 分组。
func NewDdns(s *session.Session) *Ddns {
	return &Ddns{ApiGroup: session.NewApiGroup(s)}
}

// GetDdnsList 对应 ddns/ddns-list。
func (d *Ddns) GetDdnsList() (map[string]interface{}, error) {
	return d.S.Get("ddns/ddns-list", nil, "api")
}

// GetStatus 对应 ddns/status。
func (d *Ddns) GetStatus() (map[string]interface{}, error) {
	return d.S.Get("ddns/status", nil, "api")
}

// Serverlist 对应 ddns/serverlist。
func (d *Ddns) Serverlist() (map[string]interface{}, error) {
	return d.S.Get("ddns/serverlist", nil, "api")
}