package session

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha1"
	"encoding/base64"
	"encoding/hex"
	"fmt"
	"math"
	"math/big"
	"strconv"
	"strings"
	"time"
)

// DatetimeFormat 对应 Tools.datetime_format = "%Y-%m-%d %H:%M:%S"。
const DatetimeFormat = "2006-01-02 15:04:05"

// EnforceListResponse 对应 Tools.enforce_list_response。
//
// 确保 data[plural]→[singular] 是列表：
//   - data[plural] 为 nil 时置为 {}
//   - data[plural][singular] 缺失时置为 []
//   - 若为单个 dict 则包成 [dict]
//
// data 必须是 map[string]interface{} 视图（Python dict 的 Go 等价物）。
func EnforceListResponse(data map[string]interface{}, singularKeyName string, pluralKeyName *string) map[string]interface{} {
	plural := singularKeyName + "s"
	if pluralKeyName != nil && *pluralKeyName != "" {
		plural = *pluralKeyName
	}

	var pluralMap map[string]interface{}
	if raw, ok := data[plural].(map[string]interface{}); ok {
		pluralMap = raw
	} else {
		pluralMap = map[string]interface{}{}
		data[plural] = pluralMap
	}

	var single interface{}
	if v, ok := pluralMap[singularKeyName]; ok {
		single = v
	} else {
		single = []interface{}{}
		pluralMap[singularKeyName] = single
	}

	if _, isDict := single.(map[string]interface{}); isDict {
		pluralMap[singularKeyName] = []interface{}{single}
	}

	return data
}

// RSAKey 封装公钥的 e/n。
type RSAKey struct {
	E *big.Int
	N *big.Int
}

// NewRSAKeyFromHex 从 16 进制字符串构造公钥。
func NewRSAKeyFromHex(rsaE, rsaN string) (*RSAKey, error) {
	nInt, ok := new(big.Int).SetString(rsaN, 16)
	if !ok {
		return nil, fmt.Errorf("invalid rsa_n hex: %s", rsaN)
	}
	eInt, ok := new(big.Int).SetString(rsaE, 16)
	if !ok {
		return nil, fmt.Errorf("invalid rsa_e hex: %s", rsaE)
	}
	return &RSAKey{E: eInt, N: nInt}, nil
}

// RsaEncrypt 对应 Tools.rsa_encrypt。
//
// 流程：
//  1. 对 data 做 base64 编码；
//  2. 用公钥 (n, e) 构造 RSA 公钥；
//  3. 按 padding 选择 PKCS1_v1_5(0) 或 OAEP(1)；
//  4. 将 base64 串按块大小（245/214）分块，逐块加密；
//  5. 拼接所有密文块并 hexlify；
//  6. 若 hex 串长度为偶数，原样返回；奇数则在前面补 "0"。
func RsaEncrypt(key *RSAKey, data []byte, rsaPadding int) ([]byte, error) {
	b64data := base64.StdEncoding.EncodeToString(data)

	rsaPub := &rsa.PublicKey{
		N: key.N,
		E: int(key.E.Int64()),
	}

	// 块大小
	var blockNum int
	switch rsaPadding {
	case 0:
		blockNum = 245 // PKCS1_v1_5
	case 1:
		blockNum = 214 // OAEP
	default:
		return nil, fmt.Errorf("unknown rsa_padding value %d", rsaPadding)
	}

	blocks := int(math.Ceil(float64(len(b64data)) / float64(blockNum)))
	var result []byte
	for i := 0; i < blocks; i++ {
		start := i * blockNum
		end := start + blockNum
		if end > len(b64data) {
			end = len(b64data)
		}
		block := []byte(b64data[start:end])

		var encrypted []byte
		var err error
		if rsaPadding == 0 {
			encrypted, err = rsa.EncryptPKCS1v15(rand.Reader, rsaPub, block)
		} else {
			// PyCryptodome PKCS1_OAEP.new(pubkey) 未传 hashAlgo，默认 SHA-1
			encrypted, err = rsa.EncryptOAEP(sha1.New(), rand.Reader, rsaPub, block, nil)
		}
		if err != nil {
			return nil, fmt.Errorf("rsa encrypt block: %w", err)
		}
		result = append(result, encrypted...)
	}

	hexed := make([]byte, hex.EncodedLen(len(result)))
	hex.Encode(hexed, result)
	if len(hexed)%2 == 0 {
		return hexed, nil
	}
	return append([]byte{'0'}, hexed...), nil
}

// StripDict 对应 Tools.strip_dict：只保留 wantedKeys。
func StripDict(filtered map[string]interface{}, wantedKeys ...string) map[string]interface{} {
	wanted := make(map[string]bool, len(wantedKeys))
	for _, k := range wantedKeys {
		wanted[k] = true
	}
	out := make(map[string]interface{})
	for k, v := range filtered {
		if wanted[k] {
			out[k] = v
		}
	}
	return out
}

// FilterIter 对应 Tools.filter_iter：在迭代器上按选项过滤。
// 元素为 map 或实现了 GetAttr 接口的对象。
type GetAttr interface {
	Get(attr string) (interface{}, bool)
}

// FilterIter 返回满足所有 filterOptions 条件的元素。
// data 可以是 []map[string]interface{} 或实现了 GetAttr 的元素切片。
func FilterIter(data interface{}, filterOptions map[string]interface{}, yield func(item interface{}) bool) {
	switch items := data.(type) {
	case []map[string]interface{}:
		for _, item := range items {
			if mapMatches(item, filterOptions) {
				if !yield(item) {
					return
				}
			}
		}
	case []interface{}:
		for _, item := range items {
			if itemMatches(item, filterOptions) {
				if !yield(item) {
					return
				}
			}
		}
	case []GetAttr:
		for _, item := range items {
			if attrMatches(item, filterOptions) {
				if !yield(item) {
					return
				}
			}
		}
	}
}

func mapMatches(m map[string]interface{}, f map[string]interface{}) bool {
	for attr, want := range f {
		if got, ok := m[attr]; !ok || got != want {
			return false
		}
	}
	return true
}

func itemMatches(item interface{}, f map[string]interface{}) bool {
	if m, ok := item.(map[string]interface{}); ok {
		return mapMatches(m, f)
	}
	if g, ok := item.(GetAttr); ok {
		return attrMatches(g, f)
	}
	return false
}

func attrMatches(g GetAttr, f map[string]interface{}) bool {
	for attr, want := range f {
		if got, ok := g.Get(attr); !ok || got != want {
			return false
		}
	}
	return true
}

// StringToTime 对应 Tools.string_to_datetime：
// 将 "2022-12-22 18:01:09" 解析为 time.Time。
func StringToTime(s string) (time.Time, error) {
	return time.ParseInLocation(DatetimeFormat, s, time.Local)
}

// TimeToString 对应 Tools.datetime_to_string。
func TimeToString(t time.Time) string {
	return t.Format(DatetimeFormat)
}

// MapGetString 便捷方法：从 map[string]interface{} 取字符串值
// （兼容 string 与 fmt.Stringer 数值类型）。
func MapGetString(m map[string]interface{}, key string) (string, bool) {
	v, ok := m[key]
	if !ok {
		return "", false
	}
	switch t := v.(type) {
	case string:
		return t, true
	case fmt.Stringer:
		return t.String(), true
	default:
		return "", false
	}
}

// MapGetInt 便捷方法：从 map[string]interface{} 取整数值。
func MapGetInt(m map[string]interface{}, key string) (int, bool) {
	v, ok := m[key]
	if !ok {
		return 0, false
	}
	switch t := v.(type) {
	case int:
		return t, true
	case int64:
		return int(t), true
	case float64:
		return int(t), true
	case string:
		n, err := strconv.Atoi(t)
		return n, err == nil
	default:
		return 0, false
	}
}

// StringOrNumber 返回 XML 值文本表示（等价 strconv 语义）。
func StringOrNumber(v interface{}) string {
	switch t := v.(type) {
	case string:
		return t
	case int:
		return strconv.Itoa(t)
	case int64:
		return strconv.FormatInt(t, 10)
	case float64:
		return strconv.FormatFloat(t, 'f', -1, 64)
	default:
		return fmt.Sprintf("%v", v)
	}
}

// IntToBool 等价 int(v) 后与 0 比较的 Python 真值判断。
func IntToBool(v int) bool {
	return v != 0
}

var _ = strings.TrimSpace