package api

import (
	"github.com/lvcdy/huawei-lte-api-go/enums"
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// DialUp 对应 DialUp.py。
type DialUp struct {
	*session.ApiGroup
}

// NewDialUp 创建 DialUp API 分组。
func NewDialUp(s *session.Session) *DialUp {
	return &DialUp{ApiGroup: session.NewApiGroup(s)}
}

// MobileDataswitch 获取 LTE 调制解调器开关状态。对应 dialup/mobile-dataswitch。
func (d *DialUp) MobileDataswitch() (map[string]interface{}, error) {
	return d.S.Get("dialup/mobile-dataswitch", nil, "api")
}

// Connection 获取连接设置。对应 dialup/connection。
func (d *DialUp) Connection() (map[string]interface{}, error) {
	return d.S.Get("dialup/connection", nil, "api")
}

// DialupFeatureSwitch 获取拨号功能开关状态。对应 dialup/dialup-feature-switch。
func (d *DialUp) DialupFeatureSwitch() (map[string]interface{}, error) {
	return d.S.Get("dialup/dialup-feature-switch", nil, "api")
}

// Profiles 获取拨号配置。对应 dialup/profiles。
func (d *DialUp) Profiles() (map[string]interface{}, error) {
	return d.S.Get("dialup/profiles", nil, "api")
}

// AutoApn 获取自动 APN 设置。对应 dialup/auto-apn。
func (d *DialUp) AutoApn() (map[string]interface{}, error) {
	return d.S.Get("dialup/auto-apn", nil, "api")
}

// Dial 发起拨号连接。对应 dialup/dial (post_set)。
func (d *DialUp) Dial() (interface{}, error) {
	return d.S.PostSet("dialup/dial", map[string]interface{}{
		"Action": 1,
	}, false, "api", false, false)
}

// SetMobileDataswitch 切换 LTE 调制解调器状态。对应 dialup/mobile-dataswitch (post_set)。
// dataswitch: 0 关闭，1 开启。
func (d *DialUp) SetMobileDataswitch(dataswitch int) (interface{}, error) {
	return d.S.PostSet("dialup/mobile-dataswitch", map[string]interface{}{
		"dataswitch": dataswitch,
	}, false, "api", false, false)
}

// SetDefaultProfile 设置默认拨号配置。对应 dialup/profiles (post_set, is_encrypted)。
func (d *DialUp) SetDefaultProfile(index int) (interface{}, error) {
	return d.S.PostSet("dialup/profiles", map[string]interface{}{
		"SetDefault": index,
		"Delete":     0,
		"Modify":     0,
	}, false, "api", true, false)
}

// DeleteProfile 删除拨号配置。对应 dialup/profiles (post_set, is_encrypted)。
func (d *DialUp) DeleteProfile(index int) (interface{}, error) {
	return d.S.PostSet("dialup/profiles", map[string]interface{}{
		"SetDefault": 0,
		"Delete":     index,
		"Modify":     0,
	}, false, "api", true, false)
}

// CreateProfile 创建新的拨号配置。对应 dialup/profiles (post_set, is_encrypted)。
func (d *DialUp) CreateProfile(
	name string,
	username *string,
	password *string,
	apn *string,
	dialupNumber *string,
	authMode enums.AuthMode,
	ipType enums.IpType,
	isDefault bool,
) (interface{}, error) {
	setDefault := 0
	if isDefault {
		setDefault = 1
	}
	return d.S.PostSet("dialup/profiles", map[string]interface{}{
		"SetDefault": setDefault, // E5576 上新配置总会成为默认（见上游 #221）
		"Delete":     0,
		"Modify":     1,
		"Profile": map[string]interface{}{
			"Index":        "",
			"IsValid":      1,
			"Name":         name,
			"ApnIsStatic":  intFromPtr(apn),
			"ApnName":      ptrOrNil(apn),
			"DialupNum":    ptrOrNil(dialupNumber),
			"Username":     ptrOrNil(username),
			"Password":     ptrOrNil(password),
			"AuthMode":     int(authMode),
			"IpIsStatic":   "",
			"IpAddress":    "",
			"DnsIsStatic":  "",
			"PrimaryDns":   "",
			"SecondaryDns": "",
			"ReadOnly":     "0",
			"iptype":       int(ipType),
		},
	}, false, "api", true, false)
}

// UpdateProfile 更新已有拨号配置。对应 dialup/profiles (post_set, is_encrypted)。
func (d *DialUp) UpdateProfile(
	index int,
	name string,
	username *string,
	password *string,
	apn *string,
	dialupNumber *string,
	authMode enums.AuthMode,
	ipType enums.IpType,
	isDefault bool,
) (interface{}, error) {
	setDefault := 0
	if isDefault {
		setDefault = index
	}
	return d.S.PostSet("dialup/profiles", map[string]interface{}{
		"SetDefault": setDefault,
		"Delete":     0,
		"Modify":     2,
		"Profile": map[string]interface{}{
			"Index":        index,
			"IsValid":      1,
			"Name":         name,
			"ApnIsStatic":  intFromPtr(apn),
			"ApnName":      ptrOrNil(apn),
			"DialupNum":    ptrOrNil(dialupNumber),
			"Username":     ptrOrNil(username),
			"Password":     ptrOrNil(password),
			"AuthMode":     int(authMode),
			"IpIsStatic":   "",
			"IpAddress":    "",
			"DnsIsStatic":  "",
			"PrimaryDns":   "",
			"SecondaryDns": "",
			"ReadOnly":     "0",
			"iptype":       int(ipType),
		},
	}, false, "api", true, false)
}

// SetConnectionSettings 设置连接参数。对应 dialup/connection (post_set)。
func (d *DialUp) SetConnectionSettings(
	roamAutoConnectEnable bool,
	maxIdleTime int,
	connectMode int,
	mtu int,
	autoDialSwitch bool,
	pdpAlwaysOn bool,
) (interface{}, error) {
	return d.S.PostSet("dialup/connection", map[string]interface{}{
		"RoamAutoConnectEnable": boolToInt(roamAutoConnectEnable),
		"MaxIdelTime":           maxIdleTime, // 拼写与上游一致
		"ConnectMode":           connectMode,
		"MTU":                   mtu,
		"auto_dial_switch":      boolToInt(autoDialSwitch),
		"pdp_always_on":         boolToInt(pdpAlwaysOn),
	}, false, "api", false, false)
}