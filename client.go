// Package huaweilteapi 对应 Python 版 huawei_lte_api 根包（Client.py）。
// 它聚合了所有 API 分组、config 分组与 usermanual 分组。
package huaweilteapi

import (
	"github.com/lvcdy/huawei-lte-api-go/api"
	"github.com/lvcdy/huawei-lte-api-go/config"
	"github.com/lvcdy/huawei-lte-api-go/session"
	"github.com/lvcdy/huawei-lte-api-go/usermanual"
)

// Client 对应 Client.py。
type Client struct {
	// ---- api 分组 ----
	App          *api.App
	Bluetooth    *api.Bluetooth
	Cradle       *api.Cradle
	Cwmp         *api.Cwmp
	DDns         *api.Ddns
	Developer    *api.Developer
	Device       *api.Device
	Dhcp         *api.Dhcp
	Diagnosis    *api.Diagnosis
	DialUp       *api.DialUp
	FileManager  *api.FileManager
	Global       *api.Global
	Host         *api.Host
	Lan          *api.Lan
	Language     *api.Language
	Led          *api.Led
	Log          *api.Log
	MLog         *api.MLog
	Monitoring   *api.Monitoring
	Net          *api.Net
	Ntwk         *api.Ntwk
	OnlineUpdate *api.OnlineUpdate
	Ota          *api.Ota
	Pb           *api.Pb
	Pin          *api.Pin
	Redirection  *api.Redirection
	SdCard       *api.SdCard
	Security     *api.Security
	Sms          *api.Sms
	SNtp         *api.Sntp
	Staticroute  *api.Staticroute
	Statistic    *api.Statistic
	Syslog       *api.Syslog
	System       *api.System
	Time         *api.Time
	TimeRule     *api.TimeRule
	UsbPrinter   *api.UsbPrinter
	UsbStorage   *api.UsbStorage
	User         *session.User
	Ussd         *api.Ussd
	Voice        *api.Voice
	Vpn          *api.Vpn
	VSim         *api.VSim
	WebServer    *api.WebServer
	WLan         *api.WLan

	// ---- config 分组 ----
	ConfigDialUp            *config.DialUp
	ConfigGlobal            *config.Global
	ConfigLan               *config.Lan
	ConfigNetwork           *config.Network
	ConfigPincode           *config.Pincode
	ConfigSms               *config.Sms
	ConfigVoice             *config.Voice
	ConfigWifi              *config.Wifi
	ConfigPcAssistant       *config.PcAssistant
	ConfigDeviceInformation *config.DeviceInformation
	ConfigWebUICfg          *config.WebUICfg
	ConfigDevice            *config.Device
	ConfigFastBoot          *config.FastBoot
	ConfigFirewall          *config.Firewall
	ConfigIPv6              *config.IPv6
	ConfigOta               *config.Ota
	ConfigPb                *config.Pb
	ConfigSntp              *config.Sntp
	ConfigStatistic         *config.Statistic
	ConfigStk               *config.Stk
	ConfigUpdate            *config.Update
	ConfigUPnp              *config.UPnp
	ConfigUssd              *config.Ussd
	ConfigWebSd             *config.WebSd

	// ---- usermanual 分组 ----
	UserManualPublicSysResources *usermanual.PublicSysResources
}

// NewClient 对应 Client.__init__，以 Connection 为底层会话构建全部分组。
func NewClient(c *session.Connection) *Client {
	s := c.Session
	return &Client{
		// api 分组
		App:          api.NewApp(s),
		Bluetooth:    api.NewBluetooth(s),
		Cradle:       api.NewCradle(s),
		Cwmp:         api.NewCwmp(s),
		DDns:         api.NewDdns(s),
		Developer:    api.NewDeveloper(s),
		Device:       api.NewDevice(s),
		Dhcp:         api.NewDhcp(s),
		Diagnosis:    api.NewDiagnosis(s),
		DialUp:       api.NewDialUp(s),
		FileManager:  api.NewFileManager(s),
		Global:       api.NewGlobal(s),
		Host:         api.NewHost(s),
		Lan:          api.NewLan(s),
		Language:     api.NewLanguage(s),
		Led:          api.NewLed(s),
		Log:          api.NewLog(s),
		MLog:         api.NewMLog(s),
		Monitoring:   api.NewMonitoring(s),
		Net:          api.NewNet(s),
		Ntwk:         api.NewNtwk(s),
		OnlineUpdate: api.NewOnlineUpdate(s),
		Ota:          api.NewOta(s),
		Pb:           api.NewPb(s),
		Pin:          api.NewPin(s),
		Redirection:  api.NewRedirection(s),
		SdCard:       api.NewSdCard(s),
		Security:     api.NewSecurity(s),
		Sms:          api.NewSms(s),
		SNtp:         api.NewSntp(s),
		Staticroute:  api.NewStaticroute(s),
		Statistic:    api.NewStatistic(s),
		Syslog:       api.NewSyslog(s),
		System:       api.NewSystem(s),
		Time:         api.NewTime(s),
		TimeRule:     api.NewTimeRule(s),
		UsbPrinter:   api.NewUsbPrinter(s),
		UsbStorage:   api.NewUsbStorage(s),
		User:         session.NewUser(s),
		Ussd:         api.NewUssd(s),
		Voice:        api.NewVoice(s),
		Vpn:          api.NewVpn(s),
		VSim:         api.NewVSim(s),
		WebServer:    api.NewWebServer(s),
		WLan:         api.NewWLan(s),

		// config 分组
		ConfigDialUp:            config.NewDialUp(s),
		ConfigGlobal:            config.NewGlobal(s),
		ConfigLan:               config.NewLan(s),
		ConfigNetwork:           config.NewNetwork(s),
		ConfigPincode:           config.NewPincode(s),
		ConfigSms:               config.NewSms(s),
		ConfigVoice:             config.NewVoice(s),
		ConfigWifi:              config.NewWifi(s),
		ConfigPcAssistant:       config.NewPcAssistant(s),
		ConfigDeviceInformation: config.NewDeviceInformation(s),
		ConfigWebUICfg:          config.NewWebUICfg(s),
		ConfigDevice:            config.NewDevice(s),
		ConfigFastBoot:          config.NewFastBoot(s),
		ConfigFirewall:          config.NewFirewall(s),
		ConfigIPv6:              config.NewIPv6(s),
		ConfigOta:               config.NewOta(s),
		ConfigPb:                config.NewPb(s),
		ConfigSntp:              config.NewSntp(s),
		ConfigStatistic:         config.NewStatistic(s),
		ConfigStk:               config.NewStk(s),
		ConfigUpdate:            config.NewUpdate(s),
		ConfigUPnp:              config.NewUPnp(s),
		ConfigUssd:              config.NewUssd(s),
		ConfigWebSd:             config.NewWebSd(s),

		// usermanual 分组
		UserManualPublicSysResources: usermanual.NewPublicSysResources(s),
	}
}
