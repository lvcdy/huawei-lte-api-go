package session

import (
	"encoding/base64"
	"errors"
	"io"
	"net/http"
	"net/http/httptest"
	"reflect"
	"strconv"
	"testing"
	"time"
)

const userTestTimeout = 5 * time.Second

func strPtr(s string) *string { return &s }

// 对应 tests/test_session.py 的 test_user_login_encodes_password（BASE_64 分支）。
func TestEncodePasswordBase64(t *testing.T) {
	u := NewUser(&Session{})
	got, err := u.EncodePassword("admin", strPtr("secret"), PasswordTypeBase64)
	if err != nil {
		t.Fatalf("EncodePassword: %v", err)
	}
	want := base64.StdEncoding.EncodeToString([]byte("secret"))
	if string(got) != want {
		t.Errorf("EncodePassword = %q, want %q", got, want)
	}
}

// 对应 tests/test_session.py 的 test_user_login_encodes_password（SHA256 分支）。
func TestEncodePasswordSHA256(t *testing.T) {
	s := &Session{RequestVerificationTokens: []string{"csrf-token"}}
	u := NewUser(s)
	got, err := u.EncodePassword("admin", strPtr("secret"), PasswordTypeSHA256)
	if err != nil {
		t.Fatalf("EncodePassword: %v", err)
	}
	// 期望值: base64(sha256("admin" + base64(sha256("secret").hexdigest()) + "csrf-token").hexdigest())
	want := "ODNiZmIxMzRiYWJjNjJlMDY3ZGE2NGJmNWRmYjg4ODllNTFiNzJmOTU3NzViMjQ4YWNiZWEzNGU3MjM1OTU0ZQ=="
	if string(got) != want {
		t.Errorf("EncodePassword = %q, want %q", got, want)
	}
}

// 对应 tests/test_session.py 的 test_user_login_skips_request_when_already_logged_in。
func TestUserLoginSkipsRequestWhenAlreadyLoggedIn(t *testing.T) {
	var postCount int

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch {
		case r.Method == http.MethodGet && r.URL.Path == "/":
			_, _ = io.WriteString(w, `<meta name="csrf_token" content="csrf-token">`)
		case r.Method == http.MethodGet && r.URL.Path == "/api/user/state-login":
			_, _ = io.WriteString(w, "<response><State>0</State></response>")
		case r.Method == http.MethodPost:
			postCount++
			_, _ = io.WriteString(w, "<response>OK</response>")
		default:
			t.Errorf("unexpected request %s %s", r.Method, r.URL.Path)
			w.WriteHeader(404)
		}
	}))
	defer ts.Close()

	s, err := NewSession(ts.URL+"/", userTestTimeout, &http.Client{})
	if err != nil {
		t.Fatalf("NewSession: %v", err)
	}
	defer s.Close()

	u := NewUser(s)
	ok, err := u.Login("admin", strPtr("secret"), false)
	if err != nil {
		t.Fatalf("Login: %v", err)
	}
	if !ok {
		t.Error("Login = false, want true")
	}
	if postCount != 0 {
		t.Errorf("post count = %d, want 0 (already logged in)", postCount)
	}
}

// 对应 tests/test_session.py 的 test_user_login_maps_authentication_errors。
func TestUserLoginMapsAuthenticationErrors(t *testing.T) {
	cases := []struct {
		code LoginErrorCode
		want func(error) bool
	}{
		{LoginErrorUsernameWrong, func(e error) bool {
			var target *LoginUsernameWrongError
			return errors.As(e, &target)
		}},
		{LoginErrorPasswordWrong, func(e error) bool {
			var target *LoginPasswordWrongError
			return errors.As(e, &target)
		}},
		{LoginErrorAlreadyLogin, func(e error) bool {
			var target *LoginAlreadyLoginError
			return errors.As(e, &target)
		}},
		{LoginErrorUsernamePwdWrong, func(e error) bool {
			var target *LoginUsernamePasswordWrongError
			return errors.As(e, &target)
		}},
		{LoginErrorUsernamePwdOver, func(e error) bool {
			var target *LoginUsernamePasswordOverrunError
			return errors.As(e, &target)
		}},
		{LoginErrorUsernamePwdMod, func(e error) bool {
			var target *LoginUsernamePasswordModifyError
			return errors.As(e, &target)
		}},
	}

	for _, c := range cases {
		t.Run("code-"+strconv.Itoa(int(c.code)), func(t *testing.T) {
			code := c.code
			ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				switch {
				case r.Method == http.MethodGet && r.URL.Path == "/":
					_, _ = io.WriteString(w, `<meta name="csrf_token" content="csrf-token">`)
				case r.Method == http.MethodGet && r.URL.Path == "/api/user/state-login":
					_, _ = io.WriteString(w, "<response><State>-1</State><password_type>0</password_type></response>")
				case r.Method == http.MethodPost && r.URL.Path == "/api/user/login":
					_, _ = io.WriteString(w, "<error><code>"+strconv.Itoa(int(code))+"</code><message></message></error>")
				default:
					t.Errorf("unexpected request %s %s", r.Method, r.URL.Path)
					w.WriteHeader(404)
				}
			}))
			defer ts.Close()

			s, err := NewSession(ts.URL+"/", userTestTimeout, &http.Client{})
			if err != nil {
				t.Fatalf("NewSession: %v", err)
			}
			defer s.Close()

			u := NewUser(s)
			_, err = u.Login("admin", strPtr("secret"), false)
			if err == nil {
				t.Fatalf("expected error for code %d", code)
			}
			if !c.want(err) {
				t.Errorf("code %d: err = %T (%v), want matching error type", code, err, err)
			}
			re, ok := ResponseErrorAs(err)
			if !ok {
				t.Fatalf("code %d: not a ResponseError", code)
			}
			if re.Code != ResponseCode(code) {
				t.Errorf("code %d: ResponseError.Code = %d", code, re.Code)
			}
		})
	}
}

// sanity: strPtr helper works as expected.
func TestStrPtr(t *testing.T) {
	want := "x"
	if !reflect.DeepEqual(strPtr("x"), &want) {
		t.Error("strPtr malfunction")
	}
}