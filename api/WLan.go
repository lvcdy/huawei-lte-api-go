package api

import (
	"fmt"

	"github.com/lvcdy/huawei-lte-api-go/enums"
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// WLanSettings 对应 WLan.py 的 WLanSettings dataclass。
type WLanSettings struct {
	Index              int
	Enabled            bool
	Ssid               string
	Mac                *string
	Broadcast          bool
	AuthMode           string
	ID                 *string
	RadiusKey          *string
	WpaEncryptionModes string
	WepKeyIndex        int
	GuestOffTime       int
	IsGuestNetwork     bool
}

// WLanSettingsFromDict 对应 WLanSettings.from_dict。
func WLanSettingsFromDict(data map[string]interface{}) WLanSettings {
	return WLanSettings{
		Index:              dictInt(data, "Index"),
		Enabled:            dictStr(data, "WifiEnable") == "1",
		Ssid:               dictStr(data, "WifiSsid"),
		Mac:                dictStrPtr(data, "WifiMac"),
		Broadcast:          dictStr(data, "WifiBroadcast") == "1",
		AuthMode:           dictStr(data, "WifiAuthmode"),
		WpaEncryptionModes: dictStr(data, "WifiWpaencryptionmodes"),
		WepKeyIndex:        dictInt(data, "WifiWepKeyIndex"),
		GuestOffTime:       dictInt(data, "wifiguestofftime"),
		IsGuestNetwork:     dictStr(data, "wifiisguestnetwork") == "1",
		ID:                 dictStrPtr(data, "ID"),
		RadiusKey:          dictStrPtr(data, "WifiRadiusKey"),
	}
}

// ToDict 对应 WLanSettings.to_dict。
func (w WLanSettings) ToDict() map[string]interface{} {
	mac := interface{}(nil)
	if w.Mac != nil {
		mac = *w.Mac
	}
	return map[string]interface{}{
		"Index":                  itoa(w.Index),
		"WifiEnable":             boolStr(w.Enabled),
		"WifiSsid":               w.Ssid,
		"WifiMac":                mac,
		"WifiBroadcast":          boolStr(w.Broadcast),
		"WifiAuthmode":           w.AuthMode,
		"WifiWpaencryptionmodes": w.WpaEncryptionModes,
		"WifiWepKeyIndex":        itoa(w.WepKeyIndex),
		"wifiguestofftime":       itoa(w.GuestOffTime),
	}
}

// Get 实现 session.GetAttr，供 FilterIter 使用（对应 Python getattr(data_item, attr)）。
func (w WLanSettings) Get(attr string) (interface{}, bool) {
	switch attr {
	case "Index":
		return w.Index, true
	case "enabled":
		return w.Enabled, true
	case "ssid":
		return w.Ssid, true
	case "mac":
		if w.Mac != nil {
			return *w.Mac, true
		}
		return nil, true
	case "broadcast":
		return w.Broadcast, true
	case "auth_mode":
		return w.AuthMode, true
	case "id":
		if w.ID != nil {
			return *w.ID, true
		}
		return nil, true
	case "radius_key":
		if w.RadiusKey != nil {
			return *w.RadiusKey, true
		}
		return nil, true
	case "wpa_encryption_modes":
		return w.WpaEncryptionModes, true
	case "wep_key_index":
		return w.WepKeyIndex, true
	case "guest_off_time":
		return w.GuestOffTime, true
	case "is_guest_network":
		return w.IsGuestNetwork, true
	}
	return nil, false
}

// boolStr 等价 Python 的 "1" if x else "0"。
func boolStr(b bool) string {
	if b {
		return "1"
	}
	return "0"
}

// WLan 对应 WLan.py。
type WLan struct {
	*session.ApiGroup
}

// NewWLan 创建 WLan API 分组。
func NewWLan(s *session.Session) *WLan {
	return &WLan{ApiGroup: session.NewApiGroup(s)}
}

// WifiFeatureSwitch 对应 wlan/wifi-feature-switch。
func (w *WLan) WifiFeatureSwitch() (map[string]interface{}, error) {
	return w.S.Get("wlan/wifi-feature-switch", nil, "api")
}

// StationInformation 对应 wlan/station-information。
func (w *WLan) StationInformation() (map[string]interface{}, error) {
	return w.S.Get("wlan/station-information", nil, "api")
}

// BasicSettings 对应 wlan/basic-settings。
func (w *WLan) BasicSettings() (map[string]interface{}, error) {
	return w.S.Get("wlan/basic-settings", nil, "api")
}

// SetBasicSettings 设置基本 WiFi 设置。对应 wlan/basic-settings (post_set)。
func (w *WLan) SetBasicSettings(ssid string, hide bool, wifiRestart bool) (interface{}, error) {
	return w.S.PostSet("wlan/basic-settings", session.O(
		"WifiSsid", ssid,
		"WifiHide", hide,
		"WifiRestart", boolToInt(wifiRestart),
	), false, "api", false, false)
}

// SecuritySettings 对应 wlan/security-settings。
func (w *WLan) SecuritySettings() (map[string]interface{}, error) {
	return w.S.Get("wlan/security-settings", nil, "api")
}

// SetSecuritySettings 设置安全设置。对应 wlan/security-settings (post_set)。
func (w *WLan) SetSecuritySettings(
	wpaPsk string,
	wepKey string,
	wpaEncryptionMode enums.WpaEncryptMode,
	wepEncryptionMode enums.WepEncryptMode,
	authMode enums.WlanAuthMode,
	wifiRestart bool,
) (interface{}, error) {
	return w.S.PostSet("wlan/security-settings", session.O(
		"WifiAuthmode", string(authMode),
		"WifiWepKey1", wepKey,
		"WifiWpaencryptionmodes", string(wpaEncryptionMode),
		"WifiBasicencryptionmodes", string(wepEncryptionMode),
		"WifiWpapsk", wpaPsk,
		"WifiRestart", boolToInt(wifiRestart),
	), false, "api", false, false)
}

// MultiSecuritySettings 对应 wlan/multi-security-settings。
func (w *WLan) MultiSecuritySettings() (map[string]interface{}, error) {
	return w.S.Get("wlan/multi-security-settings", nil, "api")
}

// MultiSecuritySettingsEx 对应 wlan/multi-security-settings-ex。
func (w *WLan) MultiSecuritySettingsEx() (map[string]interface{}, error) {
	return w.S.Get("wlan/multi-security-settings-ex", nil, "api")
}

// MultiBasicSettings 对应 wlan/multi-basic-settings。
func (w *WLan) MultiBasicSettings() (map[string]interface{}, error) {
	return w.S.Get("wlan/multi-basic-settings", nil, "api")
}

// SetMultiBasicSettings 设置多个 WiFi 设置。对应 wlan/multi-basic-settings (post_set)。
// clients 为形如 {"wifihostname": hostname, "WifiMacFilterMac": mac} 的字典列表。
func (w *WLan) SetMultiBasicSettings(clients []map[string]interface{}) (interface{}, error) {
	return w.S.PostSet("wlan/multi-basic-settings", map[string]interface{}{
		"Ssids": map[string]interface{}{
			"Ssid": clients,
		},
		"WifiRestart": 1,
	}, false, "api", false, false)
}

// HostList 获取主机列表。对应 wlan/host-list + enforce_list_response("Host")。
func (w *WLan) HostList() (map[string]interface{}, error) {
	hosts, err := w.S.Get("wlan/host-list", nil, "api")
	if err != nil {
		return nil, err
	}
	return session.EnforceListResponse(hosts, "Host", nil), nil
}

// HandoverSetting 对应 wlan/handover-setting。
func (w *WLan) HandoverSetting() (map[string]interface{}, error) {
	return w.S.Get("wlan/handover-setting", nil, "api")
}

// SetHandoverSetting 设置切换设置。对应 wlan/handover-setting (post_set)。
// G3_PREFER = 0, WIFI_PREFER = 2。
func (w *WLan) SetHandoverSetting(handover int) (interface{}, error) {
	return w.S.PostSet("wlan/handover-setting", map[string]interface{}{
		"Handover": handover,
	}, false, "api", false, false)
}

// MultiSwitchSettings 对应 wlan/multi-switch-settings。
func (w *WLan) MultiSwitchSettings() (map[string]interface{}, error) {
	return w.S.Get("wlan/multi-switch-settings", nil, "api")
}

// MultiMacfilterSettings 对应 wlan/multi-macfilter-settings。
func (w *WLan) MultiMacfilterSettings() (map[string]interface{}, error) {
	return w.S.Get("wlan/multi-macfilter-settings", nil, "api")
}

// SetMultiMacfilterSettings 设置多个 MAC 过滤配置。对应 wlan/multi-macfilter-settings (post_set)。
// clients 形如 {'WifiMacFilterMac0': mac, 'wifihostname0': name, 'Index': n, 'WifiMacFilterStatus': 1|2}。
func (w *WLan) SetMultiMacfilterSettings(clients []map[string]interface{}) (interface{}, error) {
	return w.S.PostSet("wlan/multi-macfilter-settings", map[string]interface{}{
		"Ssids": map[string]interface{}{
			"Ssid": clients,
		},
	}, false, "api", false, false)
}

// MultiMacfilterSettingsEx 对应 wlan/multi-macfilter-settings-ex。
func (w *WLan) MultiMacfilterSettingsEx() (map[string]interface{}, error) {
	return w.S.Get("wlan/multi-macfilter-settings-ex", nil, "api")
}

// MacFilter 对应 wlan/mac-filter。
func (w *WLan) MacFilter() (map[string]interface{}, error) {
	return w.S.Get("wlan/mac-filter", nil, "api")
}

// SetMacFilter 设置 MAC 过滤。对应 wlan/mac-filter (post_set)。
func (w *WLan) SetMacFilter(hostname string, mac string) (interface{}, error) {
	return w.S.PostSet("wlan/mac-filter", session.O(
		"wifihostname", hostname,
		"WifiMacFilterMac", mac,
	), false, "api", false, false)
}

// OledShowpassword 对应 wlan/oled-showpassword。
func (w *WLan) OledShowpassword() (map[string]interface{}, error) {
	return w.S.Get("wlan/oled-showpassword", nil, "api")
}

// Wps 对应 wlan/wps。
func (w *WLan) Wps() (map[string]interface{}, error) {
	return w.S.Get("wlan/wps", nil, "api")
}

// WpsAppin 对应 wlan/wps-appin。
func (w *WLan) WpsAppin() (map[string]interface{}, error) {
	return w.S.Get("wlan/wps-appin", nil, "api")
}

// SetWpsAppin 设置 WPS PIN。对应 wlan/wps-appin (post_set)。
func (w *WLan) SetWpsAppin(wpsappintype int, wpsappin *int) (interface{}, error) {
	pin := ""
	if wpsappin != nil {
		pin = itoa(*wpsappin)
	}
	return w.S.PostSet("wlan/wps-appin", session.O(
		"wpsappintype", wpsappintype,
		"wpsappin", pin,
	), false, "api", false, false)
}

// WpsPbc 对应 wlan/wps-pbc。
func (w *WLan) WpsPbc() (map[string]interface{}, error) {
	return w.S.Get("wlan/wps-pbc", nil, "api")
}

// SetWpsPbc 设置 WPS PBC。对应 wlan/wps-pbc (post_set)。
func (w *WLan) SetWpsPbc(wpsmode int, ssidindex int) (interface{}, error) {
	return w.S.PostSet("wlan/wps-pbc", session.O(
		"WPSMode", wpsmode,
		"ssidindex", ssidindex,
	), false, "api", false, false)
}

// WpsSwitch 对应 wlan/wps-switch。
func (w *WLan) WpsSwitch() (map[string]interface{}, error) {
	return w.S.Get("wlan/wps-switch", nil, "api")
}

// SetWpsSwitch 设置 WPS 开关。对应 wlan/wps-switch (post_set)。
func (w *WLan) SetWpsSwitch(appinenable int) (interface{}, error) {
	return w.S.PostSet("wlan/wps-switch", session.O(
		"appinenable", appinenable,
	), false, "api", false, false)
}

// StatusSwitchSettings 对应 wlan/status-switch-settings。
func (w *WLan) StatusSwitchSettings() (map[string]interface{}, error) {
	return w.S.Get("wlan/status-switch-settings", nil, "api")
}

// Wifiprofile 对应 wlan/wifiprofile。
func (w *WLan) Wifiprofile() (map[string]interface{}, error) {
	return w.S.Get("wlan/wifiprofile", nil, "api")
}

// Wififrequence 对应 wlan/wififrequence。
func (w *WLan) Wififrequence() (map[string]interface{}, error) {
	return w.S.Get("wlan/wififrequence", nil, "api")
}

// Wifiscanresult 对应 wlan/wifiscanresult。
func (w *WLan) Wifiscanresult() (map[string]interface{}, error) {
	return w.S.Get("wlan/wifiscanresult", nil, "api")
}

// WifiGuestNetworkSwitch 开关访客 WiFi 网络。对应 WLan.wifi_guest_network_switch。
func (w *WLan) WifiGuestNetworkSwitch(status bool) (interface{}, error) {
	return w.WifiNetworkSwitch(status, map[string]interface{}{"is_guest_network": true})
}

// FindWlanSettings 按条件查找 WLanSettings。对应 WLan.find_wlan_settings。
// criteria 为 key:value 映射，返回匹配的列表。
func (w *WLan) FindWlanSettings(criteria map[string]interface{}) ([]WLanSettings, error) {
	multiBasicSettings, err := w.MultiBasicSettings()
	if err != nil {
		return nil, err
	}
	ssids := toSlice(toDict(multiBasicSettings["Ssids"])["Ssid"])
	var result []WLanSettings
	for _, ssid := range ssids {
		md, ok := ssid.(map[string]interface{})
		if !ok {
			continue
		}
		item := WLanSettingsFromDict(md)
		if attrMatchesAll(item, criteria) {
			result = append(result, item)
		}
	}
	return result, nil
}

// attrMatchesAll 检查 WLanSettings 是否满足全部过滤条件（对应 Tools.filter_iter）。
func attrMatchesAll(g session.GetAttr, f map[string]interface{}) bool {
	for attr, want := range f {
		got, ok := g.Get(attr)
		if !ok || got != want {
			return false
		}
	}
	return true
}

// SaveWlanSettings 保存修改后的 WLanSettings 列表。对应 WLan.save_wlan_settings。
func (w *WLan) SaveWlanSettings(settings []WLanSettings) (interface{}, error) {
	clients := make([]map[string]interface{}, 0, len(settings))
	for _, item := range settings {
		clients = append(clients, item.ToDict())
	}
	return w.SetMultiBasicSettings(clients)
}

// WifiNetworkSwitch 按条件开关 WiFi 网络。对应 WLan.wifi_network_switch。
// status 为开关值；criteria 为 nil 时匹配所有。
func (w *WLan) WifiNetworkSwitch(status bool, criteria map[string]interface{}) (interface{}, error) {
	if criteria == nil {
		criteria = map[string]interface{}{}
	}
	items, err := w.FindWlanSettings(criteria)
	if err != nil {
		return nil, err
	}
	for i := range items {
		items[i].Enabled = status
	}
	return w.SaveWlanSettings(items)
}

// Wlandbho 对应 wlan/wlandbho。
func (w *WLan) Wlandbho() (map[string]interface{}, error) {
	return w.S.Get("wlan/wlandbho", nil, "api")
}

// WlanGuideSettings 对应 wlan/wlan-guide-settings。
func (w *WLan) WlanGuideSettings() (map[string]interface{}, error) {
	return w.S.Get("wlan/wlan-guide-settings", nil, "api")
}

// SetWlanGuideSettings 设置 WiFi 引导设置（含修改会话密码）。对应 WLan.set_wlan_guide_settings。
func (w *WLan) SetWlanGuideSettings(ssid string, wpaPsk string, currentPassword string, newPassword string) (interface{}, error) {
	guideSettings, err := w.WlanGuideSettings()
	if err != nil {
		return nil, err
	}
	ssids := toSlice(toDict(guideSettings["Ssids"])["Ssid"])

	var newSsid map[string]interface{}
	if len(ssids) > 0 {
		if md, ok := ssids[0].(map[string]interface{}); ok {
			// 拷贝一份避免修改原响应
			newSsid = map[string]interface{}{}
			for k, v := range md {
				newSsid[k] = v
			}
		}
	}
	if newSsid == nil {
		newSsid = map[string]interface{}{"Index": "0"}
	}
	newSsid["WifiSsid"] = ssid
	newSsid["WifiWpapsk"] = wpaPsk

	if err := w.S.Reload(); err != nil {
		return nil, err
	}

	data := map[string]interface{}{
		"Ssids": map[string]interface{}{
			"Ssid": []map[string]interface{}{newSsid},
		},
		"rebootInfo": map[string]interface{}{
			"isReboot": 0,
		},
		"accountInfo": map[string]interface{}{
			"currentpassword": currentPassword,
			"newpassword":     newPassword,
			"confirmpwd":      newPassword,
		},
	}
	return w.S.PostSet("wlan/wlan-guide-settings", data, true, "api", true, false)
}

// Wlanintelligent 对应 wlan/wlanintelligent。
func (w *WLan) Wlanintelligent() (map[string]interface{}, error) {
	return w.S.Get("wlan/wlanintelligent", nil, "api")
}

// GuesttimeSetting 对应 wlan/guesttime-setting。
func (w *WLan) GuesttimeSetting() (map[string]interface{}, error) {
	return w.S.Get("wlan/guesttime-setting", nil, "api")
}

// FilterMacAddresses 批量添加 MAC 地址到过滤列表。对应 WLan.filter_mac_addresses。
// filterStatus 为 '1'（白名单）或 '2'（黑名单，默认）。
func (w *WLan) FilterMacAddresses(macList []string, hostnameList []string, ssidIndex string, filterStatus string) (interface{}, error) {
	if len(macList) != len(hostnameList) {
		return nil, fmt.Errorf("the number of MAC addresses and hostnames must be the same")
	}

	clients := map[string]interface{}{
		"Index":               ssidIndex,
		"WifiMacFilterStatus": filterStatus,
	}
	for i := 0; i < len(macList); i++ {
		clients[fmt.Sprintf("WifiMacFilterMac%d", i)] = macList[i]
		clients[fmt.Sprintf("wifihostname%d", i)] = hostnameList[i]
	}
	return w.SetMultiMacfilterSettings([]map[string]interface{}{clients})
}

// extractMacHostnamePairs 从响应字典提取 MAC 与主机名配对。对应 WLan._extract_mac_hostname_pairs。
func extractMacHostnamePairs(macListDict map[string]interface{}) []map[string]string {
	var devices []map[string]string
	if macListDict == nil {
		return devices
	}
	for i := 0; ; i++ {
		mac, ok1 := macListDict[fmt.Sprintf("WifiMacFilterMac%d", i)].(string)
		hostname, _ := macListDict[fmt.Sprintf("wifihostname%d", i)].(string)
		if !ok1 {
			break
		}
		if mac != "" {
			devices = append(devices, map[string]string{"mac": mac, "hostname": hostname})
		}
	}
	return devices
}

// GetFilteredDevices 获取过滤列表中的 MAC 地址结构化列表。对应 WLan.get_filtered_devices。
func (w *WLan) GetFilteredDevices() ([]map[string]interface{}, error) {
	response, err := w.MultiMacfilterSettingsEx()
	if err != nil {
		return nil, err
	}
	var result []map[string]interface{}
	ssids := toDict(response["Ssids"])
	if ssids == nil {
		return result, nil
	}
	if _, hasSsid := ssids["Ssid"]; !hasSsid {
		return result, nil
	}
	for _, ssid := range toSlice(ssids["Ssid"]) {
		md := toDict(ssid)
		if md == nil {
			continue
		}
		ssidIndex := dictStr(md, "Index")

		var blacklistDevices []map[string]string
		if blacklist, ok := md["wifimacblacklist"].(map[string]interface{}); ok {
			blacklistDevices = extractMacHostnamePairs(blacklist)
		} else {
			blacklistDevices = []map[string]string{}
		}
		result = append(result, map[string]interface{}{
			"ssid_index":  ssidIndex,
			"filter_type": "blacklist",
			"devices":     blacklistDevices,
		})

		var whitelistDevices []map[string]string
		if whitelist, ok := md["wifimacwhitelist"].(map[string]interface{}); ok {
			whitelistDevices = extractMacHostnamePairs(whitelist)
		} else {
			whitelistDevices = []map[string]string{}
		}
		result = append(result, map[string]interface{}{
			"ssid_index":  ssidIndex,
			"filter_type": "whitelist",
			"devices":     whitelistDevices,
		})
	}
	return result, nil
}

// GetFilterStatus 获取当前 MAC 过滤状态。对应 WLan.get_filter_status。
// 返回 {"enabled": bool, "mode": "blacklist"|"whitelist"}。
func (w *WLan) GetFilterStatus() (map[string]interface{}, error) {
	response, err := w.MultiMacfilterSettingsEx()
	if err != nil {
		return nil, err
	}
	enabled := dictStr(response, "enable") == "1"
	filterStatus := dictStr(response, "wifimacfilterstatus")
	if filterStatus == "" {
		filterStatus = "2"
	}
	mode := "blacklist"
	if filterStatus == "1" {
		mode = "whitelist"
	}
	return map[string]interface{}{
		"enabled": enabled,
		"mode":    mode,
	}, nil
}
