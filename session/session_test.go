package session

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"reflect"
	"strings"
	"testing"
	"time"
)

const testTimeout = 5 * time.Second

// makeTestServer 创建一个模拟华为设备的 HTTP 服务器。
// home 为首页响应（含 csrf_token meta），其余路径返回 XML。
func makeTestServer(t *testing.T, handler http.HandlerFunc) (*httptest.Server, *Session) {
	t.Helper()
	ts := httptest.NewServer(handler)
	t.Cleanup(ts.Close)

	// 用默认 http.Client 走真实传输
	client := &http.Client{}
	s, err := NewSession(ts.URL+"/", testTimeout, client)
	if err != nil {
		t.Fatalf("NewSession: %v", err)
	}
	t.Cleanup(s.Close)
	return ts, s
}

// 对应 tests/test_session.py 的 test_post_set_sends_cesu8_xml_and_updates_csrf_token。
func TestPostSetSendsCesu8XMLAndUpdatesCSRFToken(t *testing.T) {
	var gotMethod string
	var gotURL string
	var gotHeaders http.Header
	var gotBody []byte

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.URL.Path {
		case "/":
			w.Header().Set("Content-Type", "text/html")
			_, _ = io.WriteString(w, `<meta name="csrf_token" content="csrf-token">`)
		case "/api/sms/send-sms":
			gotMethod = r.Method
			gotURL = r.URL.Path
			gotHeaders = r.Header
			gotBody, _ = io.ReadAll(r.Body)
			w.Header().Set("__RequestVerificationToken", "next-token")
			_, _ = io.WriteString(w, "<response>OK</response>")
		default:
			t.Errorf("unexpected path %q", r.URL.Path)
			w.WriteHeader(404)
		}
	}))
	defer ts.Close()

	s, err := NewSession(ts.URL+"/", testTimeout, &http.Client{})
	if err != nil {
		t.Fatalf("NewSession: %v", err)
	}
	defer s.Close()

	result, err := s.PostSet("sms/send-sms", map[string]interface{}{"Content": "A & < \U0001f600"}, false, "api", false, false)
	if err != nil {
		t.Fatalf("PostSet: %v", err)
	}
	if result != OK {
		t.Errorf("result = %v, want OK", result)
	}
	if gotMethod != http.MethodPost {
		t.Errorf("method = %s, want POST", gotMethod)
	}
	if gotURL != "/api/sms/send-sms" {
		t.Errorf("url = %s, want /api/sms/send-sms", gotURL)
	}
	if gotHeaders.Get("Content-Type") != "application/xml" {
		t.Errorf("Content-Type = %q, want application/xml", gotHeaders.Get("Content-Type"))
	}
	if gotHeaders.Get("__RequestVerificationToken") != "csrf-token" {
		t.Errorf("token header = %q, want csrf-token", gotHeaders.Get("__RequestVerificationToken"))
	}
	// CESU-8 编码后的 emoji：\xed\xa0\xbd\xed\xb8\x80
	if !bytes.Contains(gotBody, []byte("A &amp; &lt; \xed\xa0\xbd\xed\xb8\x80")) {
		t.Errorf("body = %q, want CESU-8 emoji encoded", gotBody)
	}
	// 响应头 token 被追加进 session
	if !reflect.DeepEqual(s.RequestVerificationTokens, []string{"csrf-token", "next-token"}) {
		t.Errorf("tokens = %v, want [csrf-token next-token]", s.RequestVerificationTokens)
	}
}

// 对应 tests/test_session.py 的 test_get_converts_cesu8_in_xml_response。
func TestGetConvertsCesu8InXMLResponse(t *testing.T) {
	var gotHeaders http.Header

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.URL.Path {
		case "/":
			w.Header().Set("Content-Type", "text/html")
			_, _ = io.WriteString(w, `<meta name="csrf_token" content="csrf-token">`)
		case "/api/sms/message":
			gotHeaders = r.Header
			w.Header().Set("Content-Type", "application/xml; charset=UTF-8")
			_, _ = io.WriteString(w, "<response><Message>Hello \xed\xa0\xbd\xed\xb8\x80</Message></response>")
		default:
			t.Errorf("unexpected path %q", r.URL.Path)
			w.WriteHeader(404)
		}
	}))
	defer ts.Close()

	s, err := NewSession(ts.URL+"/", testTimeout, &http.Client{})
	if err != nil {
		t.Fatalf("NewSession: %v", err)
	}
	defer s.Close()

	result, err := s.Get("sms/message", nil, "api")
	if err != nil {
		t.Fatalf("Get: %v", err)
	}
	if got := result["Message"]; got != "Hello \U0001f600" {
		t.Errorf("Message = %v, want Hello 😀", got)
	}
	if gotHeaders.Get("__RequestVerificationToken") != "csrf-token" {
		t.Errorf("token header = %q, want csrf-token", gotHeaders.Get("__RequestVerificationToken"))
	}
}

// 对应 tests/test_session.py 的 test_process_response_data_detects_json。
func TestProcessResponseDataDetectsJSON(t *testing.T) {
	content := []byte(`{"response": {"State": 1}}`)
	contentTypes := []string{"application/json; charset=UTF-8", "application/problem+json", "text/html"}

	for _, ct := range contentTypes {
		t.Run(ct, func(t *testing.T) {
			resp := &http.Response{
				Header: http.Header{"Content-Type": []string{ct}},
				Body:   io.NopCloser(bytes.NewReader(content)),
			}
			s := &Session{}
			data, err := s.processResponseData(resp)
			if err != nil {
				t.Fatalf("processResponseData: %v", err)
			}
			want := map[string]interface{}{"response": map[string]interface{}{"State": float64(1)}}
			if !reflect.DeepEqual(data, want) {
				t.Errorf("data = %v, want %v", data, want)
			}
		})
	}
}

// 对应 tests/test_session.py 的 test_process_response_data_handles_empty_response。
func TestProcessResponseDataHandlesEmptyResponse(t *testing.T) {
	resp := &http.Response{Body: io.NopCloser(bytes.NewReader(nil))}
	s := &Session{}
	data, err := s.processResponseData(resp)
	if err != nil {
		t.Fatalf("processResponseData: %v", err)
	}
	if len(data) != 0 {
		t.Errorf("data = %v, want empty map", data)
	}
}

// 对应 tests/test_session.py 的 test_process_response_data_maps_invalid_redirect_to_not_supported。
func TestProcessResponseDataMapsInvalidRedirectToNotSupported(t *testing.T) {
	// HTML 内容无法解析为 XML → 视为重定向回主页 → NotSupported
	resp := &http.Response{
		Header: http.Header{"Content-Type": []string{"text/html"}},
		Body:   io.NopCloser(strings.NewReader("<html>not valid XML")),
	}
	s := &Session{}
	data, err := s.processResponseData(resp)
	if err != nil {
		t.Fatalf("processResponseData: %v", err)
	}
	_, err = s.checkResponseStatus(data)
	if !IsNotSupported(err) {
		t.Errorf("err = %v, want NotSupportedError", err)
	}
}

// 对应 tests/test_session.py 的 test_check_response_status_maps_error_codes。
func TestCheckResponseStatusMapsErrorCodes(t *testing.T) {
	cases := []struct {
		code    string
		isError func(error) bool
	}{
		{"100001", func(e error) bool { _, ok := e.(*ResponseError); return ok }},
		{"100002", IsNotSupported},
		{"100003", IsLoginRequired},
		{"100004", IsSystemBusy},
		{"100005", func(e error) bool { _, ok := e.(*RequestFormatError); return ok }},
		{"125002", IsLoginCsrf},
		{"125003", IsWrongSessionToken},
	}
	for _, c := range cases {
		t.Run(c.code, func(t *testing.T) {
			s := &Session{}
			data := map[string]interface{}{
				"error": map[string]interface{}{"code": c.code, "message": ""},
			}
			_, err := s.checkResponseStatus(data)
			if err == nil {
				t.Fatalf("expected error for code %s", c.code)
			}
			if !c.isError(err) {
				t.Errorf("code %s: err = %T (%v), want matching error type", c.code, err, err)
			}
			re, ok := ResponseErrorAs(err)
			if !ok {
				t.Fatalf("code %s: not a ResponseError", c.code)
			}
			if ResponseCodeToStr(re.Code) != c.code {
				t.Errorf("code %s: ResponseError.Code = %v, want %s", c.code, re.Code, c.code)
			}
		})
	}
}

// 对应 tests/test_session.py 的 test_session_falls_back_to_token_endpoint。
func TestSessionFallsBackToTokenEndpoint(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.URL.Path {
		case "/":
			// 无 csrf meta
			_, _ = io.WriteString(w, "<html></html>")
		case "/api/webserver/token":
			_, _ = io.WriteString(w, "<response><token>fallback-token</token></response>")
		default:
			t.Errorf("unexpected path %q", r.URL.Path)
			w.WriteHeader(404)
		}
	}))
	defer ts.Close()

	s, err := NewSession(ts.URL+"/", 0, &http.Client{})
	if err != nil {
		t.Fatalf("NewSession: %v", err)
	}
	defer s.Close()

	if !reflect.DeepEqual(s.RequestVerificationTokens, []string{"fallback-token"}) {
		t.Errorf("tokens = %v, want [fallback-token]", s.RequestVerificationTokens)
	}
}

// 对应 tests/test_session.py 的 test_csrf_error_reloads_session_and_retries_once。
func TestCsrfErrorReloadsSessionAndRetriesOnce(t *testing.T) {
	var getCount, postCount int
	postHeaders := []http.Header{}

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch {
		case r.Method == http.MethodGet && r.URL.Path == "/":
			getCount++
			if getCount == 1 {
				_, _ = io.WriteString(w, `<meta name="csrf_token" content="first-token">`)
			} else {
				_, _ = io.WriteString(w, `<meta name="csrf_token" content="second-token">`)
			}
		case r.Method == http.MethodPost && r.URL.Path == "/api/device/control":
			postCount++
			postHeaders = append(postHeaders, r.Header.Clone())
			if postCount == 1 {
				_, _ = io.WriteString(w, "<error><code>125002</code><message></message></error>")
			} else {
				_, _ = io.WriteString(w, "<response>OK</response>")
			}
		default:
			t.Errorf("unexpected request %s %s", r.Method, r.URL.Path)
			w.WriteHeader(404)
		}
	}))
	defer ts.Close()

	s, err := NewSession(ts.URL+"/", testTimeout, &http.Client{})
	if err != nil {
		t.Fatalf("NewSession: %v", err)
	}
	defer s.Close()

	result, err := s.PostSet("device/control", map[string]interface{}{"Control": 1}, false, "api", false, false)
	if err != nil {
		t.Fatalf("PostSet: %v", err)
	}
	if result != OK {
		t.Errorf("result = %v, want OK", result)
	}
	if postCount != 2 {
		t.Errorf("post count = %d, want 2", postCount)
	}
	if postHeaders[0].Get("__RequestVerificationToken") != "first-token" {
		t.Errorf("first post token = %q, want first-token", postHeaders[0].Get("__RequestVerificationToken"))
	}
	if postHeaders[1].Get("__RequestVerificationToken") != "second-token" {
		t.Errorf("second post token = %q, want second-token", postHeaders[1].Get("__RequestVerificationToken"))
	}
}

// 对应 tests/test_session.py 的 test_connection_uses_url_credentials_but_removes_them_from_requests。
func TestConnectionUsesURLCredentialsButRemovesThemFromRequests(t *testing.T) {
	var getPaths []string
	var loginBody []byte

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch {
		case r.Method == http.MethodGet && r.URL.Path == "/":
			_, _ = io.WriteString(w, `<meta name="csrf_token" content="csrf-token">`)
		case r.Method == http.MethodGet && r.URL.Path == "/api/user/state-login":
			getPaths = append(getPaths, r.URL.Path)
			_, _ = io.WriteString(w, "<response><State>-1</State><password_type>0</password_type></response>")
		case r.Method == http.MethodPost && r.URL.Path == "/api/user/login":
			loginBody, _ = io.ReadAll(r.Body)
			_, _ = io.WriteString(w, "<response>OK</response>")
		default:
			t.Errorf("unexpected request %s %s", r.Method, r.URL.Path)
			w.WriteHeader(404)
		}
	}))
	defer ts.Close()

	// NewConnection 会执行自动登录
	_, err := NewConnection(ts.URL+"/", "admin", "secret", testTimeout, &http.Client{})
	if err != nil {
		t.Fatalf("NewConnection: %v", err)
	}
	if !reflect.DeepEqual(getPaths, []string{"/api/user/state-login"}) {
		t.Errorf("getPaths = %v, want [/api/user/state-login]", getPaths)
	}
	if !bytes.Contains(loginBody, []byte("<Username>admin</Username>")) {
		t.Errorf("login body = %q, want <Username>admin</Username>", loginBody)
	}
	if !bytes.Contains(loginBody, []byte("<Password>c2VjcmV0</Password>")) {
		t.Errorf("login body = %q, want base64 secret", loginBody)
	}
}

// 对应 tests/test_init.py 的 test_connection_wrong_url。
// Python 版 Connection("http://localhost") 抛 RequestsConnectionError；
// Go 版 NewConnection 对不可达地址应返回错误。
func TestConnectionWrongURL(t *testing.T) {
	// 用已关闭的 httptest 服务器产生连接错误
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		_, _ = io.WriteString(w, `<meta name="csrf_token" content="csrf-token">`)
	}))
	deadURL := ts.URL
	ts.Close() // 立即关闭端口 → 连接被拒绝

	if _, err := NewConnection(deadURL+"/", "", "", time.Second, nil); err == nil {
		t.Error("NewConnection(dead URL) = nil error, want connection error")
	}
}

// 辅助：输出 JSON 便于调试。
func debugJSON(v interface{}) string {
	b, _ := json.Marshal(v)
	return string(b)
}

// sanity: JSON 解析结果中数字以 float64 存在。
func TestJSONNumbersAreFloat64(t *testing.T) {
	resp := &http.Response{
		Header: http.Header{"Content-Type": []string{"application/json"}},
		Body:   io.NopCloser(strings.NewReader(`{"response": {"State": 1}}`)),
	}
	s := &Session{}
	data, err := s.processResponseData(resp)
	if err != nil {
		t.Fatal(err)
	}
	response, _ := data["response"].(map[string]interface{})
	if _, ok := response["State"].(float64); !ok {
		t.Errorf("State type = %T, want float64 (debug %s)", response["State"], debugJSON(data))
	}
}

// CookieJar 必须被自动挂载：华为新固件（如 H168-383）把 CSRF token 绑定在
// SessionID Cookie 上，不带 Cookie 的请求会被设备拒绝（125003）。
// 本测试模拟设备下发 SessionID Cookie，验证后续请求自动携带。
func TestNewSessionAutoAttachesCookieJar(t *testing.T) {
	carried := false
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.URL.Path {
		case "/":
			http.SetCookie(w, &http.Cookie{Name: "SessionID", Value: "abc123", Path: "/"})
			_, _ = io.WriteString(w, `<meta name="csrf_token" content="csrf-token">`)
		case "/api/user/state-login":
			// 首页 Cookie 必须被自动携带
			if c, err := r.Cookie("SessionID"); err == nil && c.Value == "abc123" {
				carried = true
			}
			_, _ = io.WriteString(w, "<response><State>-1</State><password_type>0</password_type></response>")
		default:
			_, _ = io.WriteString(w, "<response>OK</response>")
		}
	}))
	defer ts.Close()

	// 传一个没 Jar 的 client → NewSession 应自动补 CookieJar
	client := &http.Client{}
	s, err := NewSession(ts.URL+"/", testTimeout, client)
	if err != nil {
		t.Fatalf("NewSession: %v", err)
	}
	defer s.Close()

	if s.Client.Jar == nil {
		t.Error("Client.Jar == nil, want CookieJar auto-attached")
	}
	// 重要：不得修改调用方传入的 client（避免污染共享 DefaultClient）
	if client.Jar != nil {
		t.Error("caller's client was mutated: Jar should stay nil; NewSession must clone")
	}

	if _, err := s.Get("user/state-login", nil, "api"); err != nil {
		t.Fatalf("Get: %v", err)
	}
	if !carried {
		t.Error("SessionID cookie was not carried on subsequent request")
	}
}

// HTTP 默认客户端：传 nil 也应自动挂 Jar 且不影响 http.DefaultClient。
func TestNewSessionNilClientAttachesJarWithoutPollutingDefault(t *testing.T) {
	defBefore := http.DefaultClient.Jar
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		_, _ = io.WriteString(w, `<meta name="csrf_token" content="csrf-token">`)
	}))
	defer ts.Close()

	s, err := NewSession(ts.URL+"/", testTimeout, nil)
	if err != nil {
		t.Fatalf("NewSession: %v", err)
	}
	defer s.Close()

	if s.Client.Jar == nil {
		t.Error("Client.Jar == nil, want CookieJar auto-attached for nil client")
	}
	if http.DefaultClient.Jar != defBefore {
		t.Error("http.DefaultClient.Jar was mutated, must not pollute the shared default client")
	}
}
