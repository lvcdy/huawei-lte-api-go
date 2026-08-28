package api

import (
	"time"

	"github.com/lvcdy/huawei-lte-api-go/session"
)

// SdCard 对应 SdCard.py。
type SdCard struct {
	*session.ApiGroup
}

// NewSdCard 创建 SdCard API 分组。
func NewSdCard(s *session.Session) *SdCard {
	return &SdCard{ApiGroup: session.NewApiGroup(s)}
}

// DlnaSetting 对应 sdcard/dlna-setting。
func (s *SdCard) DlnaSetting() (map[string]interface{}, error) {
	return s.S.Get("sdcard/dlna-setting", nil, "api")
}

// SetDlnaSetting 设置 DLNA。对应 sdcard/dlna-setting (post_set)。
func (s *SdCard) SetDlnaSetting(enabled bool, shareAll bool, sharePath string) (interface{}, error) {
	return s.S.PostSet("sdcard/dlna-setting", map[string]interface{}{
		"enabled":      boolToInt(enabled),
		"sharepath":    sharePath,
		"shareallpath": boolToInt(shareAll),
	}, false, "api", false, false)
}

// Sdcard 对应 sdcard/sdcard。
func (s *SdCard) Sdcard() (map[string]interface{}, error) {
	return s.S.Get("sdcard/sdcard", nil, "api")
}

// Sdcardsamba 对应 sdcard/sdcardsamba。
func (s *SdCard) Sdcardsamba() (map[string]interface{}, error) {
	return s.S.Get("sdcard/sdcardsamba", nil, "api")
}

// SetSdcardsamba 使用 SMB 启用文件共享。对应 sdcard/sdcardsamba (post_set)。
func (s *SdCard) SetSdcardsamba(
	enabled bool,
	serverName string,
	serverDescription string,
	workgroupName string,
	anonymousAccess bool,
	printerEnabled bool,
) (interface{}, error) {
	return s.S.PostSet("sdcard/sdcardsamba", session.O(
		"enabled", boolToInt(enabled),
		"servername", serverName,
		"serverdescription", serverDescription,
		"workgroupname", workgroupName,
		"anonymousaccess", boolToInt(anonymousAccess),
		"printerenable", boolToInt(printerEnabled),
	), false, "api", false, false)
}

// Printerlist 对应 sdcard/printerlist。
func (s *SdCard) Printerlist() (map[string]interface{}, error) {
	return s.S.Get("sdcard/printerlist", nil, "api")
}

// ShareAccount 对应 sdcard/share-account。
func (s *SdCard) ShareAccount() (map[string]interface{}, error) {
	return s.S.Get("sdcard/share-account", nil, "api")
}

// Sdfile 对应 sdcard/sdfile。
func (s *SdCard) Sdfile() (map[string]interface{}, error) {
	return s.S.Get("sdcard/sdfile", nil, "api")
}

// Fileupload 对应 sdcard/fileupload。
func (s *SdCard) Fileupload() (map[string]interface{}, error) {
	return s.S.Get("sdcard/fileupload", nil, "api")
}

// CheckFileExist 对应 sdcard/Check_file_exist。
func (s *SdCard) CheckFileExist() (map[string]interface{}, error) {
	return s.S.Get("sdcard/Check_file_exist", nil, "api")
}

// CreateDir 在 SD 卡上创建目录。对应 sdcard/createdir (post_set)。
func (s *SdCard) CreateDir(name string, currentPath string, created *time.Time) (interface{}, error) {
	createTime := time.Now()
	if created != nil {
		createTime = *created
	}
	return s.S.PostSet("sdcard/createdir", map[string]interface{}{
		"CurrentPath": currentPath,
		"FileName":    name,
		"Time": map[string]interface{}{
			"Year":  createTime.Year(),
			"Month": int(createTime.Month()),
			"Day":   createTime.Day(),
			"Hour":  createTime.Hour(),
			"Min":   createTime.Minute(),
			"Sec":   createTime.Second(),
		},
	}, false, "api", false, false)
}

// DeleteFile 删除 SD 卡上的文件/目录。对应 sdcard/deletefile (post_set)。
func (s *SdCard) DeleteFile(name string, currentPath string) (interface{}, error) {
	return s.S.PostSet("sdcard/deletefile", map[string]interface{}{
		"CurrentPath":    currentPath,
		"DeleteFileList": name,
	}, false, "api", false, false)
}

// SdCapacity 获取 SD 卡容量信息。对应 sdcard/sdcapacity。
func (s *SdCard) SdCapacity() (map[string]interface{}, error) {
	return s.S.Get("sdcard/sdcapacity", nil, "api")
}
