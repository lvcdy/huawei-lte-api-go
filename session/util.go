package session

import "errors"

// errorsAs 是 errors.As 的薄封装，避免重复打字。
func errorsAs(err error, target any) bool {
	return errors.As(err, target)
}
