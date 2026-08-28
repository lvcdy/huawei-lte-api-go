package api

import (
	"strconv"

	"github.com/lvcdy/huawei-lte-api-go/session"
)

// boolToInt 将 bool 转为 1/0，等价 Python 的 `1 if x else 0`。
func boolToInt(b bool) int {
	if b {
		return 1
	}
	return 0
}

// ptrOrNil 返回指针指向的值，nil 指针返回 nil。
// 对应 Python 的 `apn`（None → XML 空元素）。
func ptrOrNil(p *string) interface{} {
	if p == nil {
		return nil
	}
	return *p
}

// intFromPtr 返回指针非 nil 时为 1，否则为 0。
// 对应 Python 的 `1 if apn else 0`。
func intFromPtr(p *string) int {
	if p != nil && *p != "" {
		return 1
	}
	return 0
}

// itoa 等价 Python 的 str(int)。
func itoa(v int) string {
	return strconv.Itoa(v)
}

// dictStr 从 map[string]interface{} 取字符串，非字符串返回 ""。
func dictStr(data map[string]interface{}, key string) string {
	if v, ok := data[key].(string); ok {
		return v
	}
	return ""
}

// dictStrPtr 取字符串指针；值缺失或非字符串返回 nil。
func dictStrPtr(data map[string]interface{}, key string) *string {
	if v, ok := data[key].(string); ok {
		return &v
	}
	return nil
}

// dictInt 取整数值（兼容 int/int64/float64/string），失败返回 0。
func dictInt(data map[string]interface{}, key string) int {
	n, _ := session.MapGetInt(data, key)
	return n
}

// toSlice 将任意值规整为 []interface{}（单值包一层，nil 返回 nil）。
func toSlice(v interface{}) []interface{} {
	switch t := v.(type) {
	case []interface{}:
		return t
	case nil:
		return nil
	default:
		return []interface{}{t}
	}
}

// toDict 断言 map[string]interface{}，否则返回 nil。
func toDict(v interface{}) map[string]interface{} {
	if m, ok := v.(map[string]interface{}); ok {
		return m
	}
	return nil
}
