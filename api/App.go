package api

import (
	"fmt"

	"github.com/lvcdy/huawei-lte-api-go/session"
)

// App 对应 App.py。
type App struct {
	*session.ApiGroup
}

// NewApp 创建 App API 分组。
func NewApp(s *session.Session) *App {
	return &App{ApiGroup: session.NewApiGroup(s)}
}

// Operatorinfo 获取运营商信息。对应 app/operatorinfo。
func (a *App) Operatorinfo(lang string) (map[string]interface{}, error) {
	return a.S.Get("app/operatorinfo", map[string]string{"lang": lang}, "api")
}

// Privacypolicy 获取隐私政策。对应 app/privacypolicy。
func (a *App) Privacypolicy(lang string) (map[string]interface{}, error) {
	return a.S.Get("app/privacypolicy", map[string]string{"lang": lang}, "api")
}

// AcceptPrivacypolicy 接受/拒绝隐私政策。对应 app/privacypolicy (post_get, is_json)。
// 成功（errcode==0）返回 "OK"，否则返回 *session.ResponseError。
func (a *App) AcceptPrivacypolicy(approve bool) (interface{}, error) {
	approveVal := "0"
	if approve {
		approveVal = "2"
	}
	response, err := a.S.PostGet("app/privacypolicy", map[string]interface{}{
		"data": map[string]interface{}{
			"Approve":  approveVal,
			"Liscence": "0", // deliberate typo, 与 Python 一致
		},
	}, false, "api", false, true)
	if err != nil {
		return nil, err
	}
	if errCode, ok := response["errcode"].(float64); ok {
		if int(errCode) == 0 {
			return session.OK, nil
		}
		return nil, &session.ResponseError{
			Code:    session.ResponseCode(int(errCode)),
			Message: fmt.Sprintf("Unexpected response: %v", response),
		}
	}
	if errCode, ok := response["errcode"].(int); ok {
		if errCode == 0 {
			return session.OK, nil
		}
		return nil, &session.ResponseError{
			Code:    session.ResponseCode(errCode),
			Message: fmt.Sprintf("Unexpected response: %v", response),
		}
	}
	return nil, &session.ResponseError{
		Code:    session.CodeSystemUnknown,
		Message: fmt.Sprintf("Unexpected response: %v", response),
	}
}
