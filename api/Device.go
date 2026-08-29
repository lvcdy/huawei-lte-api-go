package api

import (
	"github.com/lvcdy/huawei-lte-api-go/enums"
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Device 对应 Device.py。
type Device struct {
	*session.ApiGroup
}

// NewDevice 创建 Device API 分组。
func NewDevice(s *session.Session) *Device {
	return &Device{ApiGroup: session.NewApiGroup(s)}
}

// Information 获取设备信息。对应 device/information。
func (d *Device) Information() (map[string]interface{}, error) {
	return d.S.Get("device/information", nil, "api")
}

// AutorunVersion 获取 autorun 版本。对应 device/autorun-version。
func (d *Device) AutorunVersion() (map[string]interface{}, error) {
	return d.S.Get("device/autorun-version", nil, "api")
}

// DeviceFeatureSwitch 获取设备功能开关状态。对应 device/device-feature-switch。
func (d *Device) DeviceFeatureSwitch() (map[string]interface{}, error) {
	return d.S.Get("device/device-feature-switch", nil, "api")
}

// BasicInformation 获取基本设备信息。对应 device/basic_information。
func (d *Device) BasicInformation() (map[string]interface{}, error) {
	return d.S.Get("device/basic_information", nil, "api")
}

// SetBasicInformation 设置基本设备信息。对应 device/basic_information (post_set)。
func (d *Device) SetBasicInformation(restoreDefaultStatus bool) (interface{}, error) {
	return d.S.PostSet("device/basic_information", map[string]interface{}{
		"restore_default_status": boolToInt(restoreDefaultStatus),
	}, false, "api", false, false)
}

// Basicinformation 获取基本设备信息（另一端点）。对应 device/basicinformation。
func (d *Device) Basicinformation() (map[string]interface{}, error) {
	return d.S.Get("device/basicinformation", nil, "api")
}

// UsbTetheringSwitch 获取 USB 网络共享开关状态。对应 device/usb-tethering-switch。
func (d *Device) UsbTetheringSwitch() (map[string]interface{}, error) {
	return d.S.Get("device/usb-tethering-switch", nil, "api")
}

// BootTime 获取设备启动时间。对应 device/boot_time。
func (d *Device) BootTime() (map[string]interface{}, error) {
	return d.S.Get("device/boot_time", nil, "api")
}

// SetControl 控制设备电源状态。对应 device/control (post_set)。
func (d *Device) SetControl(control enums.ControlMode) (interface{}, error) {
	return d.S.PostSet("device/control", map[string]interface{}{
		"Control": int(control),
	}, false, "api", false, false)
}

// Signal 获取设备信号信息。对应 device/signal。
func (d *Device) Signal() (map[string]interface{}, error) {
	return d.S.Get("device/signal", nil, "api")
}

// AntennaStatus 获取设备天线状态。对应 device/antenna_status。
func (d *Device) AntennaStatus() (map[string]interface{}, error) {
	return d.S.Get("device/antenna_status", nil, "api")
}

// GetAntennaSettings 获取设备天线设置。对应 device/antenna_settings。
func (d *Device) GetAntennaSettings() (map[string]interface{}, error) {
	return d.S.Get("device/antenna_settings", nil, "api")
}

// SetAntennaSettings 设置设备天线设置。对应 device/antenna_settings (post_set)。
func (d *Device) SetAntennaSettings(antennaType enums.AntennaType) (interface{}, error) {
	return d.S.PostSet("device/antenna_settings", map[string]interface{}{
		"antenna_type": int(antennaType),
	}, false, "api", false, false)
}

// AntennaType 获取设备天线类型。对应 device/antenna_type。
func (d *Device) AntennaType() (map[string]interface{}, error) {
	return d.S.Get("device/antenna_type", nil, "api")
}

// AntennaSetType 获取设备天线设置类型。对应 device/antenna_set_type。
func (d *Device) AntennaSetType() (map[string]interface{}, error) {
	return d.S.Get("device/antenna_set_type", nil, "api")
}

// Logsetting 获取设备日志设置。对应 device/logsetting。
func (d *Device) Logsetting() (map[string]interface{}, error) {
	return d.S.Get("device/logsetting", nil, "api")
}

// Logport 获取设备日志端口。对应 device/logport。
func (d *Device) Logport() (map[string]interface{}, error) {
	return d.S.Get("device/logport", nil, "api")
}

// Datalock 获取设备数据锁状态。对应 device/datalock。
func (d *Device) Datalock() (map[string]interface{}, error) {
	return d.S.Get("device/datalock", nil, "api")
}

// Vendorname 获取设备厂商名称。对应 device/vendorname (post_get)。
// 注意：部分不支持此端点的设备会破坏会话，谨慎使用。
func (d *Device) Vendorname(lang string) (map[string]interface{}, error) {
	return d.S.PostGet("device/vendorname", map[string]interface{}{
		"language": lang,
	}, false, "api", false, false)
}

// Mode 设置设备模式（可开启 telnet/调试模式/生产模式，见 enums.Mode）。
// 对应 device/mode (post_set)。
func (d *Device) Mode(mode enums.Mode) (interface{}, error) {
	return d.S.PostSet("device/mode", map[string]interface{}{
		"mode": int(mode),
	}, false, "api", false, false)
}

// CompressLogfile 返回归档日志文件链接。对应 device/compresslogfile。
func (d *Device) CompressLogfile() (map[string]interface{}, error) {
	return d.S.Get("device/compresslogfile", nil, "api")
}

// SecCellInfo 获取辅小区（载波聚合 SCell）信息。对应 device/seccellinfo。
// 5G CPE 专有端点，返回类似 "ARFCN,Band,BW(可选),PCI,RSRP,RSRQ,RSSI,SINR;..."
// 的 CSV 风格字段（nrseccell_list / lteseccell_list 等）。部分固件返回空响应。
func (d *Device) SecCellInfo() (map[string]interface{}, error) {
	return d.S.Get("device/seccellinfo", nil, "api")
}

// NbrCellInfo 获取邻小区（邻区/Neighbor Cell）信息。对应 device/nbrcellinfo。
// 5G CPE 专有端点，返回类似 "ARFCN,Band,PCI,RSRP,RSRQ,RSSI,SINR;..."
// 的 CSV 风格字段（nbrcell_nrlist / nbrcell_ltelist 等）。部分固件返回空响应。
func (d *Device) NbrCellInfo() (map[string]interface{}, error) {
	return d.S.Get("device/nbrcellinfo", nil, "api")
}
