// Package enums 对应 Python 版 huawei_lte_api.enums 的枚举迁移。
package enums

// Cradle 对应 cradle.py。
type Cradle struct{}

// ConnectionStatus 对应 ConnectionStatusEnum（IntEnum）。
type ConnectionStatus int

const (
	ConnectionStatusConnecting        ConnectionStatus = 900
	ConnectionStatusConnected         ConnectionStatus = 901
	ConnectionStatusDisconnected      ConnectionStatus = 902
	ConnectionStatusDisconnecting     ConnectionStatus = 903
	ConnectionStatusConnectFailed     ConnectionStatus = 904
	ConnectionStatusConnectStatusNull ConnectionStatus = 905
	ConnectionStatusConnectStatusErr  ConnectionStatus = 906
)

// Device 对应 device.py。

// AntennaType 对应 AntennaTypeEnum。
type AntennaType int

const (
	AntennaTypeIntegrated AntennaType = 0
	AntennaTypeExternal12 AntennaType = 1
	AntennaTypeExternal1  AntennaType = 2
	AntennaTypeAuto       AntennaType = 3
)

// ControlMode 对应 ControlModeEnum。
type ControlMode int

const (
	ControlModeReboot              ControlMode = 1
	ControlModeReset               ControlMode = 2 // 恢复出厂设置
	ControlModeBackupConfiguration ControlMode = 3 // 备份配置，可从 http://192.168.8.1/nvram.bak 下载
	ControlModePowerOff            ControlMode = 4
)

// Mode 对应 ModeEnum。
type Mode int

const (
	ModeNormal       Mode = 0
	ModeDebug        Mode = 1
	ModeEnableTelnet Mode = 2
)

// DialUp 对应 dialup.py。

type AuthMode int

const (
	AuthModeAuto AuthMode = 0
	AuthModePAP  AuthMode = 1
	AuthModeCHAP AuthMode = 2
)

type IpType int

const (
	IpTypeIPv4     IpType = 0
	IpTypeIPv6     IpType = 1
	IpTypeIPv4IPv6 IpType = 2
)

// Net 对应 net.py。

// NetworkMode 对应 NetworkModeEnum（字符串枚举）。
type NetworkMode string

const (
	NetworkModeAuto     NetworkMode = "00"
	NetworkMode2GOnly   NetworkMode = "01"
	NetworkMode3GOnly   NetworkMode = "02"
	NetworkMode3G2GAuto NetworkMode = "0201"
	NetworkMode4GOnly   NetworkMode = "03"
	NetworkMode4G2GAuto NetworkMode = "0301"
	NetworkMode4G3GAuto NetworkMode = "0302"
)

// NetworkBand 对应 NetworkBandEnum。
type NetworkBand int64

const (
	NetworkBandBC0A  NetworkBand = 0x01
	NetworkBandBC0B  NetworkBand = 0x02
	NetworkBandBC1   NetworkBand = 0x04
	NetworkBandBC2   NetworkBand = 0x08
	NetworkBandBC3   NetworkBand = 0x10
	NetworkBandBC4   NetworkBand = 0x20
	NetworkBandBC5   NetworkBand = 0x40
	NetworkBandGSM18 NetworkBand = 0x80
	NetworkBandGSM9  NetworkBand = 0x300
	NetworkBandBC6   NetworkBand = 0x400
	NetworkBandBC7   NetworkBand = 0x800
	NetworkBandBC8   NetworkBand = 0x1000
	NetworkBandBC9   NetworkBand = 0x2000
	NetworkBandBC10  NetworkBand = 0x4000
	NetworkBandBC11  NetworkBand = 0x8000
	NetworkBandGSM85 NetworkBand = 0x80000
	NetworkBandGSM19 NetworkBand = 0x200000
	NetworkBandUMB1  NetworkBand = 0x400000
	NetworkBandUMB2  NetworkBand = 0x800000
	NetworkBandBC12  NetworkBand = 0x10000000
	NetworkBandBC13  NetworkBand = 0x20000000
	NetworkBandUMB5  NetworkBand = 0x4000000
	NetworkBandBC14  NetworkBand = 0x80000000
	NetworkBandUMB8  NetworkBand = 0x2000000000000

	// NetworkBandAll 单独使用，不要与其他值 OR。
	NetworkBandAll NetworkBand = 0x3FFFFFFF
)

// LTEBand 对应 LTEBandEnum。
type LTEBand int64

const (
	LTEBandB1  LTEBand = 0x01
	LTEBandB3  LTEBand = 0x04
	LTEBandB7  LTEBand = 0x40
	LTEBandB8  LTEBand = 0x80
	LTEBandB20 LTEBand = 0x80000
	LTEBandB28 LTEBand = 0x8000000
	LTEBandB38 LTEBand = 0x2000000000
	LTEBandB40 LTEBand = 0x8000000000

	// LTEBandAll 单独使用。
	LTEBandAll LTEBand = 0x7FFFFFFFFFFFFFFF
)
