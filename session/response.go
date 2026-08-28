package session

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strconv"
	"strings"
)

// processResponseData 对应 Python 版 _process_response_data。
//
// 逻辑：
//  1. 依据 Content-Type 判定 JSON/XML（/json、+json → JSON；/xml、+xml → XML）；
//  2. 若 Content-Type 无法判定，嗅探首字节 `{`/`[` → JSON；
//  3. JSON → 直接解析；
//  4. XML → 先 cesu8_fix 再解析；
//  5. XML 解析失败且存在重定向历史 → 返回 {"error": {"code": 100002, "message": ""}}。
func (s *Session) processResponseData(resp *http.Response) (map[string]interface{}, error) {
	// 读取 body
	body := make([]byte, 0)
	if resp.Body != nil {
		buf := make([]byte, 4096)
		for {
			n, err := resp.Body.Read(buf)
			body = append(body, buf[:n]...)
			if err != nil {
				break
			}
		}
	}

	isJSON := false
	haveJSON := false
	ct := resp.Header.Get("Content-Type")
	if ct != "" {
		// 去掉参数（EmailMessage 语义）：取分号前的内容
		contentType := ct
		if idx := strings.Index(contentType, ";"); idx >= 0 {
			contentType = strings.TrimSpace(contentType[:idx])
		}
		lower := strings.ToLower(contentType)
		if strings.HasSuffix(lower, "/json") || strings.HasSuffix(lower, "+json") {
			isJSON = true
			haveJSON = true
		} else if strings.HasSuffix(lower, "/xml") || strings.HasSuffix(lower, "+xml") {
			isJSON = false
			haveJSON = true
		}
	}

	// Content-Type 无法判定时嗅探内容
	if !haveJSON && len(body) > 0 && (body[0] == '{' || body[0] == '[') {
		isJSON = true
	}

	if isJSON {
		var out map[string]interface{}
		if len(body) == 0 {
			return map[string]interface{}{}, nil
		}
		if err := json.Unmarshal(body, &out); err != nil {
			return nil, fmt.Errorf("json decode: %w", err)
		}
		return out, nil
	}

	// XML 分支
	if len(body) == 0 {
		return map[string]interface{}{}, nil
	}
	fixed := Cesu8Fix(body)
	m, err := ParseXML(fixed)
	if err != nil {
		// 解析失败且存在重定向历史 → 生成 not supported 错误
		if respHasHistory(resp) {
			return map[string]interface{}{
				"error": map[string]interface{}{
					"code":    ResponseCodeToStr(CodeSystemNoSupport),
					"message": "",
				},
			}, nil
		}
		return nil, fmt.Errorf("xml parse: %w", err)
	}
	return m, nil
}

// respHasHistory 粗略判断是否存在重定向历史（等价 response.history 非空）。
// Go 的 http.Client 自动跟随重定向，不保留 history；
// 通过检查最终响应是否为 HTML（大概率是重定向回主页）近似判断。
func respHasHistory(resp *http.Response) bool {
	if resp == nil {
		return false
	}
	ct := resp.Header.Get("Content-Type")
	return strings.Contains(strings.ToLower(ct), "html")
}

// checkResponseStatus 对应 Python 版 _check_response_status。
// 返回 (值, error)。值可能是 string（"OK"）或 map。
func (s *Session) checkResponseStatus(data map[string]interface{}) (interface{}, error) {
	if _, hasErr := data["error"]; hasErr {
		rawErr, _ := data["error"]
		var errMap map[string]interface{}
		switch t := rawErr.(type) {
		case map[string]interface{}:
			errMap = t
		case string:
			// 序列化后的 error 字符串（JSON 场景）
			return nil, &ResponseError{Code: 0, Message: t}
		default:
			errMap = map[string]interface{}{}
		}

		code := 0
		if v, ok := MapGetString(errMap, "code"); ok {
			code, _ = strconv.Atoi(v)
		}
		message, _ := MapGetString(errMap, "message")
		if message == "" {
			message = errorCodeToMessage(ResponseCode(code))
		}
		return nil, checkErrorCode(ResponseCode(code), message)
	}

	response, hasResponse := data["response"]
	if hasResponse {
		if response == nil {
			return map[string]interface{}{}, nil
		}
		return response, nil
	}
	return data, nil
}

func errorCodeToMessage(code ResponseCode) string {
	switch code {
	case CodeSystemBusy:
		return "System busy"
	case CodeSystemNoRights:
		return "No rights (needs login)"
	case CodeSystemNoSupport:
		return "No support"
	case CodeSystemUnknown:
		return "Unknown"
	case CodeSystemCSRF:
		return "Session error"
	case CodeWrongSessionToken:
		return "Wrong Session Token"
	case CodeFormatError:
		return "Request format error"
	}
	return "Unknown"
}

// checkErrorCode 根据错误码产生对应的类型化错误。
func checkErrorCode(code ResponseCode, message string) error {
	base := &ResponseError{Code: code, Message: message}
	switch code {
	case CodeSystemBusy:
		return &SystemBusyError{base}
	case CodeSystemNoRights:
		return &LoginRequiredError{base}
	case CodeSystemNoSupport:
		return &NotSupportedError{base}
	case CodeSystemCSRF:
		return &LoginCsrfError{base}
	case CodeWrongSessionToken:
		return &WrongSessionTokenError{base}
	case CodeFormatError:
		return &RequestFormatError{base}
	case 108001:
		return &LoginUsernameWrongError{&LoginInvalidCredentialsError{base}}
	case 108002:
		return &LoginPasswordWrongError{&LoginInvalidCredentialsError{base}}
	case 108003:
		return &LoginAlreadyLoginError{base}
	case 108006:
		return &LoginUsernamePasswordWrongError{&LoginInvalidCredentialsError{base}}
	case 108007:
		return &LoginUsernamePasswordOverrunError{base}
	case 115002:
		return &LoginUsernamePasswordModifyError{base}
	default:
		return base
	}
}
