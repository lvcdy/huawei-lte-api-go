package enums

// SdCard 对应 sdcard.py。
type SdCard struct{}

// SdCardStatus 对应 SdCardStatus。
type SdCardStatus int

const (
	SdCardStatusNotDetected  SdCardStatus = 0
	SdCardStatusOK           SdCardStatus = 1
	SdCardStatusNotFormatted SdCardStatus = 2
)

// Sms 对应 sms.py。

// BoxType 对应 BoxTypeEnum。
type BoxType int

const (
	BoxTypeLocalInbox BoxType = 1
	BoxTypeLocalSent  BoxType = 2
	BoxTypeLocalDraft BoxType = 3
	BoxTypeLocalTrash BoxType = 4
	BoxTypeSimInbox   BoxType = 5
	BoxTypeSimSent    BoxType = 6
	BoxTypeSimDraft   BoxType = 7
	BoxTypeMixInbox   BoxType = 8
	BoxTypeMixSent    BoxType = 9
	BoxTypeMixDraft   BoxType = 10
)

// TextMode 对应 TextModeEnum。
type TextMode int

const (
	TextModeUCS2      TextMode = 0
	TextModeSevenBit  TextMode = 1
	TextModeEightBit  TextMode = 2
)

// SaveMode 对应 SaveModeEnum。
type SaveMode int

const (
	SaveModeLocal        SaveMode = 0
	SaveModeSimCard      SaveMode = 1
	SaveModeSimCardFirst SaveMode = 2
	SaveModeLocalFirst   SaveMode = 3
	SaveModeUnknown      SaveMode = 4
)

// SendType 对应 SendTypeEnum。
type SendType int

const (
	SendTypeSend         SendType = 0
	SendTypeSendAndSave  SendType = 1
)

// Priority 对应 PriorityEnum。
type Priority int

const (
	PriorityNormal    Priority = 0
	PriorityInteractive Priority = 1
	PriorityUrgent    Priority = 2
	PriorityEmergency Priority = 3
	PriorityUnknown   Priority = 4
)

// SmsType 对应 TypeEnum。
type SmsType int

const (
	SmsTypeSingle                     SmsType = 1
	SmsTypeMultipart                  SmsType = 2
	SmsTypeUnicode                    SmsType = 5
	SmsTypeDeliveryConfirmationSuccess SmsType = 7
	SmsTypeDeliveryConfirmationFailure SmsType = 8
)

// SmsStatus 对应 StatusEnum。
type SmsStatus int

const (
	SmsStatusNew          SmsStatus = 0
	SmsStatusRead         SmsStatus = 1
	SmsStatusPending      SmsStatus = 2
	SmsStatusSend         SmsStatus = 3
	SmsStatusSendFailed   SmsStatus = 4
)

// SortType 对应 SortTypeEnum。
type SortType int

const (
	SortTypeDate  SortType = 0
	SortTypePhone SortType = 1
	SortTypeIndex SortType = 2
)

// Vpn 对应 vpn.py。

// VPNType 对应 VPNType（字符串枚举）。
type VPNType string

const (
	VPNTypePPTP VPNType = "pptp"
	VPNTypeL2TP VPNType = "l2tp"
)

// Wlan 对应 wlan.py。

// WlanAuthMode 对应 AuthModeEnum。
type WlanAuthMode string

const (
	WlanAuthModeAuto     WlanAuthMode = "AUTO"
	WlanAuthModeOpen     WlanAuthMode = "OPEN"
	WlanAuthModeShare    WlanAuthMode = "SHARE"
	WlanAuthModeWpaPsk   WlanAuthMode = "WPA-PSK"
	WlanAuthModeWpa2Psk  WlanAuthMode = "WPA2-PSK"
	WlanAuthModeWpaWpa2  WlanAuthMode = "WPA/WPA2-PSK"
)

// WpaEncryptMode 对应 WpaEncryptModeEnum。
type WpaEncryptMode string

const (
	WpaEncryptModeAES  WpaEncryptMode = "AES"
	WpaEncryptModeTKIP WpaEncryptMode = "TKIP"
	WpaEncryptModeMix  WpaEncryptMode = "MIX"
)

// WepEncryptMode 对应 WepEncryptModeEnum。
type WepEncryptMode string

const (
	WepEncryptModeNone   WepEncryptMode = "NONE"
	WepEncryptModeWep    WepEncryptMode = "WEP"
	WepEncryptModeWep64  WepEncryptMode = "WEP64"
	WepEncryptModeWep128 WepEncryptMode = "WEP128"
)