// Package session 是 huawei-lte-api-go 的传输核心层，
// 对应 Python 版的 huawei_lte_api.Session / Connection / Tools / exceptions。
package session

import (
	"fmt"
	"strconv"
)

// ResponseCodeToStr 将错误码转为十进制字符串（XML 中 code 元素为字符串）。
func ResponseCodeToStr(code ResponseCode) string {
	return strconv.Itoa(int(code))
}

// ResponseCode 对应 Python 版 enums/client.py 的 ResponseCodeEnum。
type ResponseCode int

const (
	CodeSystemUnknown     ResponseCode = 100001
	CodeSystemNoSupport   ResponseCode = 100002
	CodeSystemNoRights    ResponseCode = 100003
	CodeSystemBusy        ResponseCode = 100004
	CodeFormatError       ResponseCode = 100005
	CodeVoiceBusy         ResponseCode = 120001 // Unused
	CodeWrongToken        ResponseCode = 125001 // Unused
	CodeSystemCSRF        ResponseCode = 125002
	CodeWrongSessionToken ResponseCode = 125003
)

// OK 对应 Python 版 ResponseEnum.OK。
const OK = "OK"

// ResponseError 对应 Python 版 ResponseErrorException。
type ResponseError struct {
	Code    ResponseCode
	Message string
}

func (e *ResponseError) Error() string {
	return fmt.Sprintf("%d: %s", e.Code, e.Message)
}

// 以下错误类型一一对应 Python 版 exceptions.py 中的异常层次。
// 每个类型通过 Unwrap 返回内嵌的 *ResponseError，
// 使 errors.As / errors.Is 能穿透嵌套错误链
// （例如 LoginUsernameWrongError → LoginInvalidCredentialsError → ResponseError）。

// NotSupportedError 对应 ResponseErrorNotSupportedException。
type NotSupportedError struct{ *ResponseError }

func (e *NotSupportedError) Unwrap() error { return e.ResponseError }

// LoginRequiredError 对应 ResponseErrorLoginRequiredException。
type LoginRequiredError struct{ *ResponseError }

func (e *LoginRequiredError) Unwrap() error { return e.ResponseError }

// SystemBusyError 对应 ResponseErrorSystemBusyException。
type SystemBusyError struct{ *ResponseError }

func (e *SystemBusyError) Unwrap() error { return e.ResponseError }

// LoginCsrfError 对应 ResponseErrorLoginCsrfException。
type LoginCsrfError struct{ *ResponseError }

func (e *LoginCsrfError) Unwrap() error { return e.ResponseError }

// WrongSessionTokenError 对应 ResponseErrorWrongSessionToken。
type WrongSessionTokenError struct{ *ResponseError }

func (e *WrongSessionTokenError) Unwrap() error { return e.ResponseError }

// RequestFormatError 对应 RequestFormatException。
type RequestFormatError struct{ *ResponseError }

func (e *RequestFormatError) Unwrap() error { return e.ResponseError }

// LoginInvalidCredentialsError 对应 LoginErrorInvalidCredentialsException，
// 是永久无效凭据类登录错误的基类。
type LoginInvalidCredentialsError struct{ *ResponseError }

func (e *LoginInvalidCredentialsError) Unwrap() error { return e.ResponseError }

// LoginUsernameWrongError 对应 LoginErrorUsernameWrongException。
type LoginUsernameWrongError struct{ *LoginInvalidCredentialsError }

func (e *LoginUsernameWrongError) Unwrap() error { return e.LoginInvalidCredentialsError }

// LoginPasswordWrongError 对应 LoginErrorPasswordWrongException。
type LoginPasswordWrongError struct{ *LoginInvalidCredentialsError }

func (e *LoginPasswordWrongError) Unwrap() error { return e.LoginInvalidCredentialsError }

// LoginAlreadyLoginError 对应 LoginErrorAlreadyLoginException。
type LoginAlreadyLoginError struct{ *ResponseError }

func (e *LoginAlreadyLoginError) Unwrap() error { return e.ResponseError }

// LoginUsernamePasswordWrongError 对应 LoginErrorUsernamePasswordWrongException。
type LoginUsernamePasswordWrongError struct{ *LoginInvalidCredentialsError }

func (e *LoginUsernamePasswordWrongError) Unwrap() error { return e.LoginInvalidCredentialsError }

// LoginUsernamePasswordOverrunError 对应 LoginErrorUsernamePasswordOverrunException。
type LoginUsernamePasswordOverrunError struct{ *ResponseError }

func (e *LoginUsernamePasswordOverrunError) Unwrap() error { return e.ResponseError }

// LoginUsernamePasswordModifyError 对应 LoginErrorUsernamePasswordModifyException。
type LoginUsernamePasswordModifyError struct{ *ResponseError }

func (e *LoginUsernamePasswordModifyError) Unwrap() error { return e.ResponseError }

// 便捷判断函数，等价于 Python 的 isinstance(e, XxxException)。

func IsNotSupported(err error) bool {
	var e *NotSupportedError
	return errorsAs(err, &e)
}

func IsLoginRequired(err error) bool {
	var e *LoginRequiredError
	return errorsAs(err, &e)
}

func IsSystemBusy(err error) bool {
	var e *SystemBusyError
	return errorsAs(err, &e)
}

func IsLoginCsrf(err error) bool {
	var e *LoginCsrfError
	return errorsAs(err, &e)
}

func IsWrongSessionToken(err error) bool {
	var e *WrongSessionTokenError
	return errorsAs(err, &e)
}

func IsLoginInvalidCredentials(err error) bool {
	var e *LoginInvalidCredentialsError
	return errorsAs(err, &e)
}

// ResponseErrorAs 返回任意 ResponseError 派生的错误，失败返回 false。
func ResponseErrorAs(err error) (*ResponseError, bool) {
	var e *ResponseError
	if errorsAs(err, &e) {
		return e, true
	}
	return nil, false
}

// Code 提取错误码；若错误不是 ResponseError 派生物返回 0。
func Code(err error) ResponseCode {
	if e, ok := ResponseErrorAs(err); ok {
		return e.Code
	}
	return 0
}