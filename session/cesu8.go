package session

import (
	"bytes"
	"fmt"
)

// Cesu8Encode 将字符串编码为 CESU-8 字节串。
// 对应 Python 版 Session.cesu8_encode。
//
// 规则：
//   - BMP 字符（U+0000..U+FFFF）按标准 UTF-8 编码；
//   - 增补平面字符（U+10000..U+10FFFF）拆成 UTF-16 代理对，
//     每个代理字符再按 UTF-8 算法编码（每对 6 字节：
//     \xed[\xa0-\xaf][\x80-\xbf] 高代理 + \xed[\xb0-\xbf][\x80-\xbf] 低代理）。
func Cesu8Encode(text string) ([]byte, error) {
	var out bytes.Buffer

	for _, code := range text {
		if code <= 0xFFFF {
			// 标准 UTF-8 编码 BMP 字符
			var tmp [3]byte
			n := encodeRune(tmp[:], code)
			out.Write(tmp[:n])
		} else if code <= 0x10FFFF {
			// surrogate pair, 每个用 UTF-8 算法编码
			base := code - 0x10000
			high := 0xD800 + (base >> 10)
			low := 0xDC00 + (base & 0x3FF)
			writeSurrogatePass(&out, high)
			writeSurrogatePass(&out, low)
		} else {
			return nil, &InvalidCesu8CharacterError{Code: int32(code)}
		}
	}

	return out.Bytes(), nil
}

// writeSurrogatePass 将代理字符按 UTF-8 算法编码（3 字节）。
// 等价于 Python 的 chr(c).encode(errors="surrogatepass")。
func writeSurrogatePass(out *bytes.Buffer, codeUnit rune) {
	// CESU-8 代理字符固定以 0xED 开头
	out.WriteByte(0xED)
	out.WriteByte(0x80 | byte((codeUnit>>6)&0x3F))
	out.WriteByte(0x80 | byte(codeUnit&0x3F))
}

// encodeRune 将单个 code point 按标准 UTF-8 编码写入 dst。
// 返回写入的字节数（1-3 字节，仅处理 BMP）。
func encodeRune(dst []byte, code rune) int {
	switch {
	case code < 0x80:
		dst[0] = byte(code)
		return 1
	case code < 0x800:
		dst[0] = 0xC0 | byte(code>>6)
		dst[1] = 0x80 | byte(code&0x3F)
		return 2
	default:
		dst[0] = 0xE0 | byte(code>>12)
		dst[1] = 0x80 | byte((code>>6)&0x3F)
		dst[2] = 0x80 | byte(code&0x3F)
		return 3
	}
}

// InvalidCesu8CharacterError 对应 Python 版 InvalidCesu8CharacterError。
type InvalidCesu8CharacterError struct {
	Code int32
}

func (e *InvalidCesu8CharacterError) Error() string {
	return fmt.Sprintf("Cant encode character 0x%X to CESU-8", e.Code)
}

// isHighSurrogate 判断字节序列是否代表 CESU-8 高代理（0xED 0xA0-0xAF）。
func isHighSurrogate(b []byte) bool {
	return len(b) >= 3 && b[0] == 0xED && b[1]&0xF0 == 0xA0
}

// isLowSurrogate 判断字节序列是否代表 CESU-8 低代理（0xED 0xB0-0xBF）。
func isLowSurrogate(b []byte) bool {
	return len(b) >= 3 && b[0] == 0xED && b[1]&0xF0 == 0xB0
}

// Cesu8Fix 将 CESU-8 编码的字节串转换为标准 UTF-8。
// 对应 Python 版 Session.cesu8_fix。
//
// 逻辑：扫描 6 字节的代理对（高代理+低代理），
// 合并为增补平面码点并用标准 UTF-8 编码。
func Cesu8Fix(blob []byte) []byte {
	out := make([]byte, 0, len(blob))
	i := 0
	for i < len(blob) {
		// 尝试匹配 6 字节代理对：ED A0-AF .. ED B0-BF ..
		if i+5 < len(blob) &&
			isHighSurrogate(blob[i:]) && blob[i+2]&0xC0 == 0x80 &&
			isLowSurrogate(blob[i+3:]) && blob[i+5]&0xC0 == 0x80 {
			// 高代理单元：b1(0xED) b2(0xA0-0xAF) b3(0x80-0xBF)
			// 低代理单元：b4(0xED) b5(0xB0-0xBF) b6(0x80-0xBF)
			high := rune(0xD800) | (rune(blob[i+1]&0x0F) << 6) | rune(blob[i+2]&0x3F)
			low := rune(0xDC00) | (rune(blob[i+4]&0x0F) << 6) | rune(blob[i+5]&0x3F)
			code := 0x10000 + ((high - 0xD800) << 10) + (low - 0xDC00)

			// 4 字节 UTF-8 编码
			out = append(out, 0xF0|byte(code>>18),
				0x80|byte((code>>12)&0x3F),
				0x80|byte((code>>6)&0x3F),
				0x80|byte(code&0x3F))
			i += 6
			continue
		}
		out = append(out, blob[i])
		i++
	}
	return out
}
