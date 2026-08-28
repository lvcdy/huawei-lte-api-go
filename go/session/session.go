package session

import (
	"fmt"
	"io"
	"net/http"
	"net/http/cookiejar"
	"net/url"
	"regexp"
	"strings"
	"time"
)

// csrfRe 对应 Python 版 csrf_re：
// name="csrf_token" content="..."
var csrfRe = regexp.MustCompile(`name="csrf_token"\s+content="(\S+)"`)

// Session 对应 Python 版 huawei_lte_api.Session。
type Session struct {
	URL                       string
	Timeout                   time.Duration
	Client                    *http.Client
	RequestVerificationTokens []string
	encryptionKey             map[string]interface{}
	CustomRequestsSession     bool
}

// NewSession 创建 Session 并初始化 CSRF tokens。
// url 中内嵌的认证信息会被剥离（rpartition("@")[-1]）。
// timeout 为 0 表示不设置超时。
//
// 与 Python 版 requests.Session 一致，requestsClient 会被强制挂载 CookieJar：
// 华为新固件（如 H168-383）将 CSRF token 绑定在 SessionID Cookie 上，若客户端
// 不带 Cookie，设备会一律返回 125003 (Wrong Session Token)。
func NewSession(rawURL string, timeout time.Duration, requestsClient *http.Client) (*Session, error) {
	// 记录调用方是否提供了 client（决定 Close 是否释放连接）。
	customRequestsSession := requestsClient != nil
	// 注意：http.DefaultClient 是全局共享的，不能直接改它；复制一份再挂 Jar。
	if requestsClient == nil {
		requestsClient = &http.Client{}
	}
	if requestsClient.Jar == nil {
		jar, err := cookiejar.New(nil)
		if err != nil {
			return nil, fmt.Errorf("create cookiejar: %w", err)
		}
		// 浅拷贝，避免修改调用方全局 client 的字段。
		clone := *requestsClient
		clone.Jar = jar
		requestsClient = &clone
	}
	if timeout > 0 {
		requestsClient.Timeout = timeout
	}

	parsed, err := url.Parse(rawURL)
	if err != nil {
		return nil, fmt.Errorf("parse url: %w", err)
	}
	// 剥离认证信息：netloc.rpartition("@")[-1]
	stripAuth := parsed.Host
	if idx := strings.LastIndex(parsed.Host, "@"); idx >= 0 {
		stripAuth = parsed.Host[idx+1:]
	}
	scheme := parsed.Scheme
	if scheme == "" {
		scheme = "http"
	}
	// Python urlunparse((scheme, netloc, *parsed[2:])) 保留 path/params/query/fragment
	rest := parsed.EscapedPath()
	if rest == "" {
		rest = "/"
	}
	if parsed.RawQuery != "" {
		rest += "?" + parsed.RawQuery
	}
	if parsed.Fragment != "" {
		rest += "#" + parsed.Fragment
	}
	clearURL := scheme + "://" + stripAuth + rest

	if !strings.HasSuffix(clearURL, "/") {
		clearURL += "/"
	}

	s := &Session{
		URL:                   clearURL,
		Timeout:               timeout,
		Client:                requestsClient,
		CustomRequestsSession: customRequestsSession,
	}

	if err := s.initializeCSRFTokensAndSession(); err != nil {
		if !s.CustomRequestsSession {
			s.Close()
		}
		return nil, err
	}
	return s, nil
}

// Close 对应 Python 版 Session.close。
func (s *Session) Close() {
	if !s.CustomRequestsSession {
		// http.Client 无显式 Close（底层 transport 可 CloseIdleConnections）
		if tr, ok := s.Client.Transport.(*http.Transport); ok {
			tr.CloseIdleConnections()
		}
	}
}

// Reload 对应 Python 版 Session.reload。
func (s *Session) Reload() error {
	return s.initializeCSRFTokensAndSession()
}

// initializeCSRFTokensAndSession 对应 _initialize_csrf_tokens_and_session。
func (s *Session) initializeCSRFTokensAndSession() error {
	s.RequestVerificationTokens = nil

	resp, err := s.doGet(s.URL, nil, nil)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	csrfTokens := csrfRe.FindAllStringSubmatch(string(body), -1)
	if len(csrfTokens) > 0 {
		for _, m := range csrfTokens {
			s.RequestVerificationTokens = append(s.RequestVerificationTokens, m[1])
		}
	} else {
		token, err := s.getToken()
		if err != nil {
			return err
		}
		if token != nil {
			s.RequestVerificationTokens = append(s.RequestVerificationTokens, *token)
		}
	}
	return nil
}

// GetToken 对应 _get_token。
func (s *Session) getToken() (*string, error) {
	data, err := s.Get("webserver/token", nil, "api")
	if err != nil {
		if IsNotSupported(err) {
			data2, err2 := s.Get("webserver/SesTokInfo", nil, "api")
			if err2 != nil {
				if IsNotSupported(err2) {
					return nil, nil
				}
				return nil, err2
			}
			if tok, ok := MapGetString(data2, "TokInfo"); ok {
				return &tok, nil
			}
			return nil, nil
		}
		return nil, err
	}
	if tok, ok := MapGetString(data, "token"); ok {
		return &tok, nil
	}
	return nil, nil
}

// buildFinalURL 对应 _build_final_url。
func (s *Session) buildFinalURL(endpoint, prefix string) string {
	if prefix == "" {
		prefix = "api"
	}
	return s.URL + prefix + "/" + endpoint
}

// doGet 执行 GET（不检查响应状态）。
func (s *Session) doGet(requestURL string, params map[string]string, headers map[string]string) (*http.Response, error) {
	req, err := http.NewRequest(http.MethodGet, requestURL, nil)
	if err != nil {
		return nil, err
	}
	if params != nil {
		q := req.URL.Query()
		for k, v := range params {
			q.Set(k, v)
		}
		req.URL.RawQuery = q.Encode()
	}
	for k, v := range headers {
		req.Header.Set(k, v)
	}
	return s.Client.Do(req)
}

// Get 对应 Python 版 Session.get（带 @_try_or_reload_and_retry）。
func (s *Session) Get(endpoint string, params map[string]string, prefix string) (map[string]interface{}, error) {
	data, err := s.getWithRetry(endpoint, params, prefix)
	return data, err
}

// getWithRetry 实现 GET + 重试逻辑：
// 捕获 LoginCsrf → reload → 重试一次。
func (s *Session) getWithRetry(endpoint string, params map[string]string, prefix string) (map[string]interface{}, error) {
	data, err := s.getOnce(endpoint, params, prefix)
	if err != nil {
		if IsLoginCsrf(err) {
			if rerr := s.Reload(); rerr != nil {
				return nil, rerr
			}
			return s.getOnce(endpoint, params, prefix)
		}
		return nil, err
	}
	return data, nil
}

// getOnce 执行单次 GET 请求。
func (s *Session) getOnce(endpoint string, params map[string]string, prefix string) (map[string]interface{}, error) {
	headers := map[string]string{}
	if len(s.RequestVerificationTokens) == 1 {
		headers["__RequestVerificationToken"] = s.RequestVerificationTokens[0]
	}

	resp, err := s.doGet(s.buildFinalURL(endpoint, prefix), params, headers)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	data, err := s.processResponseData(resp)
	if err != nil {
		return nil, err
	}
	raw, err := s.checkResponseStatus(data)
	if err != nil {
		return nil, err
	}
	return coerceMap(raw, endpoint)
}

// coerceMap 将 checkResponseStatus 的结果转为 map。
// 对应 Python 版 get() 返回 dict 的约定；若响应为字符串（罕见），
// 包装为 {"response": str} 以保持调用方 dict 语义。
func coerceMap(raw interface{}, endpoint string) (map[string]interface{}, error) {
	switch v := raw.(type) {
	case map[string]interface{}:
		return v, nil
	case string:
		return map[string]interface{}{"response": v}, nil
	case []interface{}:
		return map[string]interface{}{"response": v}, nil
	default:
		return nil, fmt.Errorf("get %s: unexpected response type %T", endpoint, raw)
	}
}

// PostGet 对应 Python 版 post_get。
func (s *Session) PostGet(endpoint string, data interface{}, refreshCSRF bool, prefix string, isEncrypted bool, isJSON bool) (map[string]interface{}, error) {
	resp, err := s.postWithRetry(endpoint, data, refreshCSRF, prefix, isEncrypted, isJSON)
	if err != nil {
		return nil, err
	}
	m, ok := resp.(map[string]interface{})
	if !ok {
		return nil, fmt.Errorf("post_get: expected dict response, got %T", resp)
	}
	return m, nil
}

// PostSet 对应 Python 版 post_set。
func (s *Session) PostSet(endpoint string, data interface{}, refreshCSRF bool, prefix string, isEncrypted bool, isJSON bool) (interface{}, error) {
	resp, err := s.postWithRetry(endpoint, data, refreshCSRF, prefix, isEncrypted, isJSON)
	if err != nil {
		return nil, err
	}
	return resp, nil
}

// postWithRetry 实现 POST + CSRF 重试。
func (s *Session) postWithRetry(endpoint string, data interface{}, refreshCSRF bool, prefix string, isEncrypted, isJSON bool) (interface{}, error) {
	resp, err := s.postOnce(endpoint, data, refreshCSRF, prefix, isEncrypted, isJSON)
	if err != nil {
		if IsLoginCsrf(err) {
			if rerr := s.Reload(); rerr != nil {
				return nil, rerr
			}
			return s.postOnce(endpoint, data, refreshCSRF, prefix, isEncrypted, isJSON)
		}
		return nil, err
	}
	return resp, nil
}

// postOnce 执行单次 POST 请求。
func (s *Session) postOnce(endpoint string, data interface{}, refreshCSRF bool, prefix string, isEncrypted, isJSON bool) (interface{}, error) {
	headers := map[string]string{}
	if isEncrypted {
		headers["Content-Type"] = "application/x-www-form-urlencoded; charset=UTF-8;enc"
		headers["encrypt_transmit"] = "encrypt_transmit"
	} else {
		headers["Content-Type"] = "application/xml"
	}

	if len(s.RequestVerificationTokens) > 0 {
		if len(s.RequestVerificationTokens) > 1 {
			// pop(0)：取第一个并移除
			headers["__RequestVerificationToken"] = s.RequestVerificationTokens[0]
			s.RequestVerificationTokens = s.RequestVerificationTokens[1:]
		} else {
			headers["__RequestVerificationToken"] = s.RequestVerificationTokens[0]
		}
	}

	var body []byte
	var err error
	if isJSON {
		body, err = JSONMarshal(data)
		if err != nil {
			return nil, err
		}
	} else if data != nil {
		body, err = CreateRequestXML(data)
		if err != nil {
			return nil, err
		}
	} else {
		body = []byte{}
	}

	if isEncrypted {
		body, err = s.encryptData(body)
		if err != nil {
			return nil, err
		}
	}

	resp, err := s.doPost(s.buildFinalURL(endpoint, prefix), body, headers)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return nil, &HTTPError{Code: resp.StatusCode, Status: resp.Status}
	}

	if refreshCSRF {
		s.RequestVerificationTokens = nil
	}

	// 提取响应头中的 CSRF tokens
	if v := resp.Header.Get("__RequestVerificationTokenone"); v != "" {
		s.RequestVerificationTokens = append(s.RequestVerificationTokens, v)
		if v2 := resp.Header.Get("__RequestVerificationTokentwo"); v2 != "" {
			s.RequestVerificationTokens = append(s.RequestVerificationTokens, v2)
		}
	} else if v := resp.Header.Get("__RequestVerificationToken"); v != "" {
		s.RequestVerificationTokens = append(s.RequestVerificationTokens, v)
	}

	dataMap, err := s.processResponseData(resp)
	if err != nil {
		return nil, err
	}
	return s.checkResponseStatus(dataMap)
}

// PostFile 对应 Python 版 post_file（multipart 上传）。
func (s *Session) PostFile(endpoint string, fileField string, fileName string, fileData []byte, data map[string]string, prefix string) (string, error) {
	if len(s.RequestVerificationTokens) > 0 {
		if data == nil {
			data = map[string]string{}
		}
		data["csrf_token"] = s.RequestVerificationTokens[0]
	}

	var buf strings.Builder
	boundary := "----GoFormBoundary"
	buf.WriteString("--" + boundary + "\r\n")
	for k, v := range data {
		buf.WriteString(fmt.Sprintf("Content-Disposition: form-data; name=%q\r\n\r\n%s\r\n", k, v))
		buf.WriteString("--" + boundary + "\r\n")
	}
	buf.WriteString(fmt.Sprintf("Content-Disposition: form-data; name=%q; filename=%q\r\n", fileField, fileName))
	buf.WriteString("Content-Type: application/octet-stream\r\n\r\n")

	var payload []byte
	payload = append(payload, buf.String()...)
	payload = append(payload, fileData...)
	payload = append(payload, []byte("\r\n--"+boundary+"--\r\n")...)

	req, err := http.NewRequest(http.MethodPost, s.buildFinalURL(endpoint, prefix), strings.NewReader(string(payload)))
	if err != nil {
		return "", err
	}
	req.Header.Set("Content-Type", "multipart/form-data; boundary="+boundary)

	resp, err := s.Client.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return "", &HTTPError{Code: resp.StatusCode, Status: resp.Status}
	}

	content, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}
	return strings.ToLower(string(content)), nil
}

// HTTPError 非 2xx 响应错误（对应 requests.raise_for_status）。
type HTTPError struct {
	Code   int
	Status string
}

func (e *HTTPError) Error() string {
	return fmt.Sprintf("%d %s", e.Code, e.Status)
}

// JSONMarshal 序列化 JSON（data 为 nil 时输出 "null"）。
func JSONMarshal(data interface{}) ([]byte, error) {
	if data == nil {
		return []byte("null"), nil
	}
	return jsonMarshal(data)
}

// doPost 执行 POST。
func (s *Session) doPost(requestURL string, body []byte, headers map[string]string) (*http.Response, error) {
	req, err := http.NewRequest(http.MethodPost, requestURL, strings.NewReader(string(body)))
	if err != nil {
		return nil, err
	}
	for k, v := range headers {
		req.Header.Set(k, v)
	}
	return s.Client.Do(req)
}

// encryptData 对应 _encrypt_data。
func (s *Session) encryptData(data []byte) ([]byte, error) {
	rsaPadding := s.getRSAPadding()
	pubkeyData := s.getEncryptionKey()
	rsaE, _ := MapGetString(pubkeyData, "encpubkeye")
	rsaN, _ := MapGetString(pubkeyData, "encpubkeyn")
	if rsaE == "" || rsaN == "" {
		return nil, fmt.Errorf("no pub key was found")
	}
	key, err := NewRSAKeyFromHex(rsaE, rsaN)
	if err != nil {
		return nil, err
	}
	return RsaEncrypt(key, data, rsaPadding)
}

// getEncryptionKey 对应 _get_encryption_key。
func (s *Session) getEncryptionKey() map[string]interface{} {
	if s.encryptionKey == nil {
		key, err := s.Get("webserver/publickey", nil, "api")
		if err == nil {
			s.encryptionKey = key
		} else {
			s.encryptionKey = map[string]interface{}{}
		}
	}
	return s.encryptionKey
}

// getRSAPadding 对应 _get_rsa_padding。
func (s *Session) getRSAPadding() int {
	stateLogin, err := s.Get("user/state-login", nil, "api")
	if err != nil {
		return 0
	}
	if v, ok := MapGetString(stateLogin, "rsapadingtype"); ok {
		n, err := parseInt(v)
		if err == nil {
			return n
		}
	}
	return 0
}

func parseInt(s string) (int, error) {
	var n int
	_, err := fmt.Sscanf(s, "%d", &n)
	if n == 0 && err != nil {
		return 0, err
	}
	return n, nil
}