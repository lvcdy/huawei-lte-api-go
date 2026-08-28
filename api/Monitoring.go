package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Monitoring 对应 Monitoring.py。
type Monitoring struct {
	*session.ApiGroup
}

// NewMonitoring 创建 Monitoring API 分组。
func NewMonitoring(s *session.Session) *Monitoring {
	return &Monitoring{ApiGroup: session.NewApiGroup(s)}
}

// ConvergedStatus 获取路由器汇聚状态。对应 monitoring/converged-status。
func (m *Monitoring) ConvergedStatus() (map[string]interface{}, error) {
	return m.S.Get("monitoring/converged-status", nil, "api")
}

// Status 获取路由器状态信息。对应 monitoring/status。
// 注意：部分路由器的信号可能在 SignalIcon 或 SignalIconNr 字段中。
func (m *Monitoring) Status() (map[string]interface{}, error) {
	return m.S.Get("monitoring/status", nil, "api")
}

// CheckNotifications 检查通知。对应 monitoring/check-notifications。
func (m *Monitoring) CheckNotifications() (map[string]interface{}, error) {
	return m.S.Get("monitoring/check-notifications", nil, "api")
}

// TrafficStatistics 获取流量统计。对应 monitoring/traffic-statistics。
func (m *Monitoring) TrafficStatistics() (map[string]interface{}, error) {
	return m.S.Get("monitoring/traffic-statistics", nil, "api")
}

// StartDate 获取监控开始日期。对应 monitoring/start_date。
func (m *Monitoring) StartDate() (map[string]interface{}, error) {
	return m.S.Get("monitoring/start_date", nil, "api")
}

// SetStartDate 设置 LTE 网络用量告警。对应 monitoring/start_date (post_set)。
func (m *Monitoring) SetStartDate(startDay int, dataLimit string, monthThreshold int) (interface{}, error) {
	return m.S.PostSet("monitoring/start_date", session.O(
		"StartDay", startDay,
		"DataLimit", dataLimit,
		"MonthThreshold", monthThreshold,
		"SetMonthData", 1,
	), false, "api", false, false)
}

// StartDateWlan 获取 WLAN 监控开始日期。对应 monitoring/start_date_wlan。
func (m *Monitoring) StartDateWlan() (map[string]interface{}, error) {
	return m.S.Get("monitoring/start_date_wlan", nil, "api")
}

// SetStartDateWlan 设置 WLAN 网络用量告警。对应 monitoring/start_date_wlan (post_set)。
func (m *Monitoring) SetStartDateWlan(startDay int, dataLimit string, monthThreshold int) (interface{}, error) {
	return m.S.PostSet("monitoring/start_date_wlan", session.O(
		"StartDay", startDay,
		"DataLimit", dataLimit,
		"MonthThreshold", monthThreshold,
		"SettingEnable", 1,
	), false, "api", false, false)
}

// MonthStatistics 获取月度统计。对应 monitoring/month_statistics。
func (m *Monitoring) MonthStatistics() (map[string]interface{}, error) {
	return m.S.Get("monitoring/month_statistics", nil, "api")
}

// MonthStatisticsWlan 获取 WLAN 月度统计。对应 monitoring/month_statistics_wlan。
func (m *Monitoring) MonthStatisticsWlan() (map[string]interface{}, error) {
	return m.S.Get("monitoring/month_statistics_wlan", nil, "api")
}

// SetClearTraffic 清除流量统计。对应 monitoring/clear-traffic (post_set)。
func (m *Monitoring) SetClearTraffic() (interface{}, error) {
	return m.S.PostSet("monitoring/clear-traffic", map[string]interface{}{
		"ClearTraffic": 1,
	}, false, "api", false, false)
}

// WifiMonthSetting 对应 monitoring/wifi-month-setting。
func (m *Monitoring) WifiMonthSetting() (map[string]interface{}, error) {
	return m.S.Get("monitoring/wifi-month-setting", nil, "api")
}

// DailyDataLimit 获取日数据上限。对应 monitoring/daily-data-limit。
func (m *Monitoring) DailyDataLimit() (map[string]interface{}, error) {
	return m.S.Get("monitoring/daily-data-limit", nil, "api")
}

// StatisticFeatureSwitch 获取统计功能开关状态。对应 monitoring/statistic-feature-switch。
func (m *Monitoring) StatisticFeatureSwitch() (map[string]interface{}, error) {
	return m.S.Get("monitoring/statistic-feature-switch", nil, "api")
}

// OnekeyDiag 获取一键诊断状态。对应 monitoring/onekey_diag。
func (m *Monitoring) OnekeyDiag() (map[string]interface{}, error) {
	return m.S.Get("monitoring/onekey_diag", nil, "api")
}
