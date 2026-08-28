package api

import (
	"fmt"
	"path/filepath"
	"strings"

	"github.com/lvcdy/huawei-lte-api-go/session"
)

// FileManager 对应 FileManager.py。
type FileManager struct {
	*session.ApiGroup
}

// NewFileManager 创建 FileManager API 分组。
func NewFileManager(s *session.Session) *FileManager {
	return &FileManager{ApiGroup: session.NewApiGroup(s)}
}

// Upload 上传固件并触发升级。对应 filemanager/upload (post_file)。
// 仅允许 *.bin 或 *.zip 文件。
func (f *FileManager) Upload(fileData []byte, fileName string) (string, error) {
	ext := strings.ToLower(filepath.Ext(fileName))
	if ext != ".bin" && ext != ".zip" {
		return "", fmt.Errorf("Only *.bin or *.zip is allowed")
	}
	base := filepath.Base(fileName)
	return f.S.PostFile("filemanager/upload", "uploadfile", base, fileData, map[string]string{
		"cur_path": "OU:" + base,
	}, "api")
}
