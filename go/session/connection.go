package session

import (
	"net/http"
	"net/url"
	"time"
)

// ApiGroup 对应 Python 版 huawei_lte_api.ApiGroup：
// 持有 Session，API 分组都嵌入它并转发请求。
type ApiGroup struct {
	S *Session
}

// NewApiGroup 构造 ApiGroup。
func NewApiGroup(s *Session) *ApiGroup {
	return &ApiGroup{S: s}
}

// Connection 对应 Python 版 huawei_lte_api.Connection（Session 子类）。
// 会在构造时自动处理 URL 内嵌凭据并（可选）登录。
type Connection struct {
	*Session
	UserSession *UserSession
}

// NewConnection 对应 Connection.__init__。
//
// username/password 缺省时从 URL 的 userinfo 中提取；
// 任一凭据存在则自动创建 UserSession（强制登录）。
func NewConnection(rawURL string, username, password string, timeout time.Duration, requestsClient *http.Client) (*Connection, error) {
	parsed, err := url.Parse(rawURL)
	if err != nil {
		return nil, err
	}

	// User login code：username = username or parsed_url.username
	if username == "" && parsed.User != nil {
		username = parsed.User.Username()
	}
	if password == "" && parsed.User != nil {
		pw, _ := parsed.User.Password()
		password = pw
	}

	s, err := NewSession(rawURL, timeout, requestsClient)
	if err != nil {
		return nil, err
	}

	c := &Connection{Session: s}
	if username != "" || password != "" {
		uname := username
		if uname == "" {
			uname = DEFAULT_USERNAME
		}
		us, err := NewUserSession(s, uname, password)
		if err != nil {
			if !s.CustomRequestsSession {
				s.Close()
			}
			return nil, err
		}
		c.UserSession = us
	}
	return c, nil
}

// Close 对应 Connection.close：先关 UserSession 再关 Session。
// UserSession.Close 已静默处理 LoginRequired/NotSupported。
func (c *Connection) Close() {
	if c.UserSession != nil {
		c.UserSession.Close()
	}
	c.Session.Close()
}