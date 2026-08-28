package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Language 对应 Language.py。
type Language struct {
	*session.ApiGroup
}

// NewLanguage 创建 Language API 分组。
func NewLanguage(s *session.Session) *Language {
	return &Language{ApiGroup: session.NewApiGroup(s)}
}

// SetCurrentLanguage 设置当前语言。对应 language/current-language (post_set)。
func (l *Language) SetCurrentLanguage(lang string) (interface{}, error) {
	return l.S.PostSet("language/current-language", map[string]interface{}{
		"CurrentLanguage": lang,
	}, false, "api", false, false)
}

// CurrentLanguage 获取当前语言。对应 language/current-language。
func (l *Language) CurrentLanguage() (map[string]interface{}, error) {
	return l.S.Get("language/current-language", nil, "api")
}