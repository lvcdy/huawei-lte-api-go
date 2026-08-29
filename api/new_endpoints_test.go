package api

import (
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/lvcdy/huawei-lte-api-go/session"
)

const apiTestTimeout = 5 * time.Second

// newMockSession 创建带 mock 服务器的会话测试环境。
// handler 负责验证请求并回写 XML；返回 mock 服务器、Session 与请求记录器。
func newMockSession(t *testing.T, handler http.HandlerFunc) (*httptest.Server, *session.Session, *[]string) {
	t.Helper()
	var requests []string

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		requests = append(requests, r.Method+" "+r.URL.Path)
		switch r.URL.Path {
		case "/":
			w.Header().Set("Content-Type", "text/html")
			_, _ = io.WriteString(w, `<meta name="csrf_token" content="csrf-token">`)
			return
		default:
			if handler != nil {
				handler(w, r)
				return
			}
			w.Header().Set("Content-Type", "application/xml")
			_, _ = io.WriteString(w, "<response>OK</response>")
		}
	}))
	t.Cleanup(ts.Close)

	s, err := session.NewSession(ts.URL+"/", apiTestTimeout, &http.Client{})
	if err != nil {
		t.Fatalf("NewSession: %v", err)
	}
	t.Cleanup(s.Close)

	return ts, s, &requests
}

// TestSecCellInfo 验证 GET api/device/seccellinfo。
func TestSecCellInfo(t *testing.T) {
	_, s, reqs := newMockSession(t, func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/device/seccellinfo" {
			_, _ = io.WriteString(w, `<?xml version="1.0" encoding="UTF-8"?><response><nrseccell_list>123,78,0,501,440,-80,-95,-11,8</nrseccell_list><cell_id>0</cell_id></response>`)
		}
	})
	data, err := NewDevice(s).SecCellInfo()
	if err != nil {
		t.Fatalf("SecCellInfo: %v", err)
	}
	if got := mustStr(t, data, "nrseccell_list"); got != "123,78,0,501,440,-80,-95,-11,8" {
		t.Errorf("nrseccell_list = %q", got)
	}
	last := lastRequest(reqs)
	if last != "GET /api/device/seccellinfo" {
		t.Errorf("request = %s", last)
	}
}

// TestNbrCellInfo 验证 GET api/device/nbrcellinfo。
func TestNbrCellInfo(t *testing.T) {
	_, s, reqs := newMockSession(t, func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/device/nbrcellinfo" {
			_, _ = io.WriteString(w, `<response><nbrcell_nrlist>123,78,0,501,440,-80,-95,-11,8;124,79,1,502,441,-75,-90,-10,9</nbrcell_nrlist></response>`)
		}
	})
	data, err := NewDevice(s).NbrCellInfo()
	if err != nil {
		t.Fatalf("NbrCellInfo: %v", err)
	}
	if got := mustStr(t, data, "nbrcell_nrlist"); !strings.Contains(got, ";") {
		t.Errorf("nbrcell_nrlist = %q, want multi-row CSV", got)
	}
	if last := lastRequest(reqs); last != "GET /api/device/nbrcellinfo" {
		t.Errorf("request = %s", last)
	}
}

// TestAntennaConfiguration 验证 GET api/net/antenna-configuration。
func TestAntennaConfiguration(t *testing.T) {
	_, s, reqs := newMockSession(t, func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/net/antenna-configuration" {
			_, _ = io.WriteString(w, `<response><antenna_mode>0</antenna_mode><gain>3.5</gain></response>`)
		}
	})
	data, err := NewNet(s).AntennaConfiguration()
	if err != nil {
		t.Fatalf("AntennaConfiguration: %v", err)
	}
	if got := mustStr(t, data, "gain"); got != "3.5" {
		t.Errorf("gain = %q", got)
	}
	if last := lastRequest(reqs); last != "GET /api/net/antenna-configuration" {
		t.Errorf("request = %s", last)
	}
}

// TestLockCell 验证 POST api/net/lock-cell 及参数顺序。
func TestLockCell(t *testing.T) {
	var body string
	_, s, reqs := newMockSession(t, func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/net/lock-cell" {
			b, _ := io.ReadAll(r.Body)
			body = string(b)
		}
	})
	if _, err := NewNtwk(s).LockCell(1, 1450, 501); err != nil {
		t.Fatalf("LockCell: %v", err)
	}
	if last := lastRequest(reqs); last != "POST /api/net/lock-cell" {
		t.Errorf("request = %s", last)
	}
	for _, want := range []string{"<LockCell>1</LockCell>", "<Freq>1450</Freq>", "<PCI>501</PCI>"} {
		if !strings.Contains(body, want) {
			t.Errorf("body missing %s; got %s", want, body)
		}
	}
}

// TestLockCellUnlock 验证解锁模式参数。
func TestLockCellUnlock(t *testing.T) {
	var body string
	_, s, _ := newMockSession(t, func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/net/lock-cell" {
			b, _ := io.ReadAll(r.Body)
			body = string(b)
		}
	})
	if _, err := NewNtwk(s).LockCell(0, 0, 0); err != nil {
		t.Fatalf("LockCell: %v", err)
	}
	for _, want := range []string{"<LockCell>0</LockCell>", "<Freq>0</Freq>", "<PCI>0</PCI>"} {
		if !strings.Contains(body, want) {
			t.Errorf("body missing %s; got %s", want, body)
		}
	}
}

// TestDeveloperMode 验证 GET developermode/developer-mode 与 developer-item。
func TestDeveloperMode(t *testing.T) {
	_, s, reqs := newMockSession(t, func(w http.ResponseWriter, r *http.Request) {
		switch r.URL.Path {
		case "/api/developermode/developer-mode":
			_, _ = io.WriteString(w, `<response><developer_mode>1</developer_mode></response>`)
		case "/api/developermode/developer-item":
			_, _ = io.WriteString(w, `<response><telnet_enable>1</telnet_enable></response>`)
		}
	})
	dm, err := NewDeveloper(s).DeveloperMode()
	if err != nil {
		t.Fatalf("DeveloperMode: %v", err)
	}
	if got := mustStr(t, dm, "developer_mode"); got != "1" {
		t.Errorf("developer_mode = %q", got)
	}
	di, err := NewDeveloper(s).DeveloperItem()
	if err != nil {
		t.Fatalf("DeveloperItem: %v", err)
	}
	if got := mustStr(t, di, "telnet_enable"); got != "1" {
		t.Errorf("telnet_enable = %q", got)
	}
	if last := lastRequest(reqs); last != "GET /api/developermode/developer-item" {
		t.Errorf("last request = %s", last)
	}
}

// TestSetAtportStatus 验证 POST api/app/atport-status 载荷。
func TestSetAtportStatus(t *testing.T) {
	var body string
	_, s, reqs := newMockSession(t, func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/app/atport-status" {
			b, _ := io.ReadAll(r.Body)
			body = string(b)
		}
	})
	if _, err := NewDeveloper(s).SetAtportStatus(1); err != nil {
		t.Fatalf("SetAtportStatus: %v", err)
	}
	if last := lastRequest(reqs); last != "POST /api/app/atport-status" {
		t.Errorf("request = %s", last)
	}
	if !strings.Contains(body, "<enable>1</enable>") {
		t.Errorf("body missing enable=1; got %s", body)
	}
}

// TestWlanDebug 验证 GET/POST api/wlan/wlan-debug。
func TestWlanDebug(t *testing.T) {
	var body string
	_, s, reqs := newMockSession(t, func(w http.ResponseWriter, r *http.Request) {
		switch r.URL.Path {
		case "/api/wlan/wlan-debug":
			if r.Method == http.MethodPost {
				b, _ := io.ReadAll(r.Body)
				body = string(b)
			} else {
				_, _ = io.WriteString(w, `<response><telnet_enable>1</telnet_enable><developermode_enable>1</developermode_enable></response>`)
			}
		}
	})

	data, err := NewWLan(s).WlanDebug()
	if err != nil {
		t.Fatalf("WlanDebug: %v", err)
	}
	if got := mustStr(t, data, "telnet_enable"); got != "1" {
		t.Errorf("telnet_enable = %q", got)
	}

	if _, err := NewWLan(s).SetWlanDebug(map[string]interface{}{"telnet_enable": 0, "developermode_enable": 1}); err != nil {
		t.Fatalf("SetWlanDebug: %v", err)
	}
	if !strings.Contains(body, "<telnet_enable>0</telnet_enable>") || !strings.Contains(body, "<developermode_enable>1</developermode_enable>") {
		t.Errorf("body = %s", body)
	}
	if last := lastRequest(reqs); last != "POST /api/wlan/wlan-debug" {
		t.Errorf("last request = %s", last)
	}
}

func lastRequest(reqs *[]string) string {
	if reqs == nil || len(*reqs) == 0 {
		return ""
	}
	return (*reqs)[len(*reqs)-1]
}

// mustStr 取 map 字符串字段，缺失或类型不符时测试直接失败。
func mustStr(t *testing.T, m map[string]interface{}, key string) string {
	t.Helper()
	s, ok := session.MapGetString(m, key)
	if !ok {
		t.Fatalf("missing key %q in %v", key, m)
	}
	return s
}
