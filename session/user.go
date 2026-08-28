package session

import (
	"crypto/sha256"
	"encoding/base64"
	"encoding/hex"
	"fmt"
	"time"
)

// ---- enums/user.py 迁移 ----

// PasswordType 对应 PasswordTypeEnum。
type PasswordType int

const (
	PasswordTypeBase64                    PasswordType = 0
	PasswordTypeBase64AfterPasswordChange PasswordType = 3
	PasswordTypeSHA256                    PasswordType = 4
)

// LoginErrorCode 对应 LoginErrorEnum。
type LoginErrorCode int

const (
	LoginErrorUsernameWrong    LoginErrorCode = 108001
	LoginErrorPasswordWrong    LoginErrorCode = 108002
	LoginErrorAlreadyLogin     LoginErrorCode = 108003
	LoginErrorUsernamePwdWrong LoginErrorCode = 108006
	LoginErrorUsernamePwdOver  LoginErrorCode = 108007
	LoginErrorUsernamePwdMod   LoginErrorCode = 115002
)

// LoginState 对应 LoginStateEnum。
type LoginState int

const (
	LoginStateLoggedIn  LoginState = 0
	LoginStateLoggedOut LoginState = -1
	LoginStateRepeat    LoginState = -2
)

const DEFAULT_USERNAME = "admin"

// User 对应 api/User.py 的 User（ApiGroup 子类）。
type User struct {
	*ApiGroup
}

// NewUser 构造 User。
func NewUser(s *Session) *User {
	return &User{ApiGroup: NewApiGroup(s)}
}

// StateLogin 对应 state_login。
func (u *User) StateLogin() (map[string]interface{}, error) {
	return u.S.Get("user/state-login", nil, "api")
}

// stateLoginWithRetry 对应 _state_login_with_retry：
// 5 次尝试，ConnectionError 时 sleep((i+1)/10) 后重试。
func (u *User) stateLoginWithRetry() (map[string]interface{}, error) {
	const tries = 5
	for i := 0; i < tries; i++ {
		data, err := u.StateLogin()
		if err == nil {
			return data, nil
		}
		// 仅 ConnectionError 触发重试；HTTP/协议错误直接返回
		if !isConnErr(err) {
			return nil, err
		}
		if i == tries-1 {
			return nil, err
		}
		time.Sleep(time.Duration(i+1) * 100 * time.Millisecond) // (i+1)/10 秒
	}
	return nil, &ResponseError{Code: 0, Message: "Tries exhausted"}
}

// isConnErr 判断是否为连接类错误（对应 requests.exceptions.ConnectionError）。
func isConnErr(err error) bool {
	// 网络层错误（dial、reset、EOF 等）都会包装 net.OpError / io.ErrUnexpectedEOF
	// return errors.Is(err, net.ErrClosed) —— 不通用；这里直接按"不存在网络错误类型"简化：
	// 我们通过 errorsAs 检查是否有 ResponseError；若有，则不是连接错误。
	var re *ResponseError
	return !errorsAs(err, &re)
}

// EncodePassword 对应 _encode_password。
func (u *User) EncodePassword(username string, password *string, passwordType PasswordType) ([]byte, error) {
	if password == nil || *password == "" {
		return []byte{}, nil
	}

	if passwordType == PasswordTypeSHA256 {
		// 1. sha256(password) → hexdigest (.encode("ascii"))
		// 2. base64(hexdigest)
		// 3. concentrated = username + base64hex + csrf_token
		// 4. sha256(concentrated) → hexdigest → base64
		pwHash := sha256.Sum256([]byte(*password))
		pwHex := []byte(hex.EncodeToString(pwHash[:]))
		b64Hex := base64.StdEncoding.EncodeToString(pwHex)

		csrf := ""
		if len(u.S.RequestVerificationTokens) > 0 {
			csrf = u.S.RequestVerificationTokens[0]
		}
		concentrated := string(username) + b64Hex + csrf

		concHash := sha256.Sum256([]byte(concentrated))
		concHex := []byte(hex.EncodeToString(concHash[:]))
		return []byte(base64.StdEncoding.EncodeToString(concHex)), nil
	}

	// BASE_64
	return []byte(base64.StdEncoding.EncodeToString([]byte(*password))), nil
}

// login 对应 _login。
func (u *User) login(username string, password *string, passwordType PasswordType) (bool, error) {
	passwordEncoded, err := u.EncodePassword(username, password, passwordType)
	if err != nil {
		return false, err
	}

	result, err := u.S.PostSet("user/login", map[string]interface{}{
		"Username":      username,
		"Password":      string(passwordEncoded),
		"password_type": int(passwordType),
	}, true, "api", false, false)
	if err != nil {
		var re *ResponseError
		if errorsAs(err, &re) {
			code := re.Code
			message := loginErrorMessage(code)
			return false, loginErrorFromCode(code, message, err)
		}
		return false, err
	}

	return fmt.Sprint(result) == OK, nil
}

// loginErrorMessage 错误码 → 消息文本。
func loginErrorMessage(code ResponseCode) string {
	switch LoginErrorCode(code) {
	case LoginErrorUsernameWrong:
		return "Username wrong"
	case LoginErrorPasswordWrong:
		return "Password wrong"
	case LoginErrorAlreadyLogin:
		return "Already login"
	case LoginErrorUsernamePwdWrong:
		return "Username and Password wrong"
	case LoginErrorUsernamePwdOver:
		return "Password overrun"
	case LoginErrorUsernamePwdMod:
		return "Password modify"
	}
	return "Unknown"
}

// loginErrorFromCode 错误码 → 类型化异常（等价 error_code_to_exception）。
func loginErrorFromCode(code ResponseCode, message string, cause error) error {
	excMsg := fmt.Sprintf("%d: %s", code, message)
	switch LoginErrorCode(code) {
	case LoginErrorUsernameWrong:
		return &LoginUsernameWrongError{&LoginInvalidCredentialsError{&ResponseError{Code: code, Message: excMsg}}}
	case LoginErrorPasswordWrong:
		return &LoginPasswordWrongError{&LoginInvalidCredentialsError{&ResponseError{Code: code, Message: excMsg}}}
	case LoginErrorAlreadyLogin:
		return &LoginAlreadyLoginError{&ResponseError{Code: code, Message: excMsg}}
	case LoginErrorUsernamePwdWrong:
		return &LoginUsernamePasswordWrongError{&LoginInvalidCredentialsError{&ResponseError{Code: code, Message: excMsg}}}
	case LoginErrorUsernamePwdOver:
		return &LoginUsernamePasswordOverrunError{&ResponseError{Code: code, Message: excMsg}}
	case LoginErrorUsernamePwdMod:
		return &LoginUsernamePasswordModifyError{&ResponseError{Code: code, Message: excMsg}}
	}
	return &ResponseError{Code: code, Message: excMsg}
}

// Login 对应 login。
func (u *User) Login(username string, password *string, forceNewLogin bool) (bool, error) {
	if username == "" { // <= 1.6.4 向后兼容
		username = DEFAULT_USERNAME
	}

	stateLogin, err := u.stateLoginWithRetry()
	if err != nil {
		if IsNotSupported(err) {
			return true, nil
		}
		return false, err
	}

	stateStr, _ := MapGetString(stateLogin, "State")
	state, _ := parseInt(stateStr)
	if LoginState(state) == LoginStateLoggedIn && !forceNewLogin {
		return true, nil
	}

	passwordType := PasswordTypeBase64
	if ptStr, ok := MapGetString(stateLogin, "password_type"); ok {
		if n, err := parseInt(ptStr); err == nil {
			passwordType = PasswordType(n)
		}
	}
	return u.login(username, password, passwordType)
}

// Logout 对应 logout。
func (u *User) Logout() (interface{}, error) {
	return u.S.PostSet("user/logout", map[string]interface{}{"Logout": 1}, false, "api", false, false)
}

// Remind / Password / Pwd / SetPwd / SetRemind 等简单端点。
// （供外层 api 触发；这里仅实现 Session 生命周期所需的。）

// ---- UserSession ----

// UserSession 对应 api/User.py 的 UserSession。
type UserSession struct {
	user *User
}

// NewUserSession 创建并强制登录。
func NewUserSession(s *Session, username string, password string) (*UserSession, error) {
	if username == "" {
		username = DEFAULT_USERNAME
	}
	u := NewUser(s)
	ok, err := u.Login(username, &password, true)
	if err != nil {
		return nil, err
	}
	if !ok {
		return nil, &ResponseError{Code: 0, Message: "Login failed"}
	}
	return &UserSession{user: u}, nil
}

// Close 对应 UserSession.close：静默忽略 LoginRequired / NotSupported。
func (us *UserSession) Close() {
	_, err := us.user.Logout()
	if err != nil && !IsLoginRequired(err) && !IsNotSupported(err) {
		_ = err // 忽略其他错误
	}
}

// SessionAPI 快捷方法（供 Connection 使用）。
func (us *UserSession) Session() *Session {
	return us.user.S
}

// ---- api/User.py 的其余端点 ----

// Remind 对应 remind。
func (u *User) Remind() (map[string]interface{}, error) {
	return u.S.Get("user/remind", nil, "api")
}

// Password 对应 password。
func (u *User) Password() (map[string]interface{}, error) {
	return u.S.Get("user/password", nil, "api")
}

// Pwd 对应 pwd。
func (u *User) Pwd() (map[string]interface{}, error) {
	return u.S.Get("user/pwd", nil, "api")
}

// SetPwd 对应 set_pwd。
func (u *User) SetPwd() (interface{}, error) {
	return u.S.PostSet("user/pwd", map[string]interface{}{
		"module": "wlan",
		"nonce":  "aaaaaaa",
	}, false, "api", false, false)
}

// SetRemind 对应 set_remind。
func (u *User) SetRemind(remindState string) (interface{}, error) {
	return u.S.PostSet("user/remind", map[string]interface{}{
		"remindstate": remindState,
	}, false, "api", false, false)
}

// AuthenticationLogin 对应 authentication_login。
func (u *User) AuthenticationLogin() (map[string]interface{}, error) {
	return u.S.Get("user/authentication_login", nil, "api")
}

// ChallengeLogin 对应 challenge_login。
func (u *User) ChallengeLogin() (map[string]interface{}, error) {
	return u.S.Get("user/challenge_login", nil, "api")
}

// HilinkLogin 对应 hilink_login。
func (u *User) HilinkLogin() (map[string]interface{}, error) {
	return u.S.Get("user/hilink_login", nil, "api")
}

// HistoryLogin 对应 history-login。
func (u *User) HistoryLogin() (map[string]interface{}, error) {
	return u.S.Get("user/history-login", nil, "api")
}

// Heartbeat 对应 heartbeat。
func (u *User) Heartbeat() (map[string]interface{}, error) {
	return u.S.Get("user/heartbeat", nil, "api")
}

// WebFeatureSwitch 对应 web-feature-switch。
func (u *User) WebFeatureSwitch() (map[string]interface{}, error) {
	return u.S.Get("user/web-feature-switch", nil, "api")
}

// InputEvent 对应 input_event。
func (u *User) InputEvent() (map[string]interface{}, error) {
	return u.S.Get("user/input_event", nil, "api")
}

// ScreenState 对应 screen_state。
func (u *User) ScreenState() (map[string]interface{}, error) {
	return u.S.Get("user/screen_state", nil, "api")
}

// SessionState 对应 session。
func (u *User) SessionState() (map[string]interface{}, error) {
	return u.S.Get("user/session", nil, "api")
}

// SecondLogin 对应 second_login。
func (u *User) SecondLogin() (map[string]interface{}, error) {
	return u.S.Get("user/second_login", nil, "api")
}

// RememberPwd 对应 remember-pwd。
func (u *User) RememberPwd() (map[string]interface{}, error) {
	return u.S.Get("user/remember-pwd", nil, "api")
}

// Rule 对应 rule。
func (u *User) Rule() (map[string]interface{}, error) {
	return u.S.Get("user/rule", nil, "api")
}
