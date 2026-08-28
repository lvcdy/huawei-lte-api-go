package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Diagnosis 对应 Diagnosis.py。
type Diagnosis struct {
	*session.ApiGroup
}

// NewDiagnosis 创建 Diagnosis API 分组。
func NewDiagnosis(s *session.Session) *Diagnosis {
	return &Diagnosis{ApiGroup: session.NewApiGroup(s)}
}

// TraceRouteResult 对应 diagnosis/tracerouteresult。
func (d *Diagnosis) TraceRouteResult() (map[string]interface{}, error) {
	return d.S.Get("diagnosis/tracerouteresult", nil, "api")
}

// DiagnosePing 对应 diagnosis/diagnose_ping。
func (d *Diagnosis) DiagnosePing() (map[string]interface{}, error) {
	return d.S.Get("diagnosis/diagnose_ping", nil, "api")
}

// SetDiagnosePing 对应 diagnosis/diagnose_ping (post_set)。
func (d *Diagnosis) SetDiagnosePing(host string, timeout int) (interface{}, error) {
	return d.S.PostSet("diagnosis/diagnose_ping", map[string]interface{}{
		"Host":    host,
		"Timeout": timeout,
	}, false, "api", false, false)
}

// DiagnoseTraceroute 对应 diagnosis/diagnose_traceroute。
func (d *Diagnosis) DiagnoseTraceroute() (map[string]interface{}, error) {
	return d.S.Get("diagnosis/diagnose_traceroute", nil, "api")
}

// SetDiagnoseTraceroute 对应 diagnosis/diagnose_traceroute (post_set)。
func (d *Diagnosis) SetDiagnoseTraceroute(host string, timeout int, maxHopCount int) (interface{}, error) {
	return d.S.PostSet("diagnosis/diagnose_traceroute", map[string]interface{}{
		"Host":        host,
		"MaxHopCount": maxHopCount,
		"Timeout":     timeout,
	}, false, "api", false, false)
}

// TimeReboot 对应 diagnosis/time_reboot。
func (d *Diagnosis) TimeReboot() (map[string]interface{}, error) {
	return d.S.Get("diagnosis/time_reboot", nil, "api")
}

// WanServiceName 对应 diagnosis/get-wan-service-name。
func (d *Diagnosis) WanServiceName() (map[string]interface{}, error) {
	return d.S.Get("diagnosis/get-wan-service-name", nil, "api")
}