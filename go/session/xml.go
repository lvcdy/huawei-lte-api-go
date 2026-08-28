package session

import (
	"bytes"
	"encoding/xml"
	"fmt"
	"io"
	"sort"
	"strconv"
	"strings"
)

// ODict 保序字典，等价于 Python 版的 OrderedDict。
// 用于构造请求体时保持字段顺序（华为设备对部分端点字段顺序敏感）。
type ODict struct {
	keys []string
	vals []interface{}
}

// O 便捷构造：ODict 字面量。用法 O("a", 1, "b", "x")。
func O(pairs ...interface{}) *ODict {
	d := &ODict{}
	for i := 0; i+1 < len(pairs); i += 2 {
		d.Set(pairs[i].(string), pairs[i+1])
	}
	return d
}

// Set 追加或覆盖 key（保持首次插入顺序）。
func (d *ODict) Set(key string, val interface{}) {
	for i, k := range d.keys {
		if k == key {
			d.vals[i] = val
			return
		}
	}
	d.keys = append(d.keys, key)
	d.vals = append(d.vals, val)
}

// Keys 返回 key 的有序切片（副本）。
func (d *ODict) Keys() []string {
	out := make([]string, len(d.keys))
	copy(out, d.keys)
	return out
}

// Get 返回 key 的值。
func (d *ODict) Get(key string) (interface{}, bool) {
	for i, k := range d.keys {
		if k == key {
			return d.vals[i], true
		}
	}
	return nil, false
}

// Contains 报告 key 是否存在。
func (d *ODict) Contains(key string) bool {
	_, ok := d.Get(key)
	return ok
}

// Len 返回条目数。
func (d *ODict) Len() int {
	return len(d.keys)
}

// ParseXML 解析 XML 字节流为 map[string]interface{} 树，
// 等价 Python：xmltodict.parse(data, dict_constructor=dict)。
//
// 语义：
//   - 重复同名子元素 → []interface{}
//   - 叶子元素 → string
//   - 有子元素的元素 → map[string]interface{}
//   - 属性 → "@attr" 键（并入元素 dict，xmltodict 的 cdata/属性合并行为）
//   - 元素既有文本又有子元素 → "#text" 键（xmltodict 行为）
//   - 纯空白文本忽略
//
// 返回根元素名到内容的映射，如 {"response": {...}}。
func ParseXML(data []byte) (map[string]interface{}, error) {
	dec := xml.NewDecoder(bytes.NewReader(data))
	dec.Strict = false
	dec.CharsetReader = charsetReader

	var root *xmlNode
	var stack []*xmlNode
	var textBuf strings.Builder

	for {
		tok, err := dec.Token()
		if err == io.EOF {
			break
		}
		if err != nil {
			return nil, fmt.Errorf("xml parse: %w", err)
		}
		switch t := tok.(type) {
		case xml.StartElement:
			textBuf.Reset()
			node := &xmlNode{
				name:  t.Name.Local,
				attrs: t.Attr,
			}
			if len(stack) == 0 {
				root = node
			} else {
				parent := stack[len(stack)-1]
				parent.children = append(parent.children, node)
			}
			stack = append(stack, node)
		case xml.CharData:
			textBuf.Write(t)
		case xml.EndElement:
			if len(stack) == 0 {
				continue
			}
			node := stack[len(stack)-1]
			node.text = textBuf.String()
			textBuf.Reset()
			stack = stack[:len(stack)-1]
		}
	}

	if root == nil {
		return map[string]interface{}{}, nil
	}

	out := map[string]interface{}{}
	if len(root.children) == 0 {
		out[root.name] = root.text
	} else {
		out[root.name] = nodeToDict(root)
	}
	return out, nil
}

// xmlNode 内部解析节点。
type xmlNode struct {
	name     string
	text     string
	children []*xmlNode
	attrs    []xml.Attr
}

// nodeToDict 将节点转为 dict 值：
//   - 有子元素 → map[string]interface{}（含 @attr 与 #text）
//   - 无子元素 → string
func nodeToDict(n *xmlNode) interface{} {
	if len(n.children) == 0 {
		if len(n.attrs) == 0 {
			return n.text
		}
		// 叶子带属性：xmltodict 返回 {"@attr":..., "#text":...}
		m := map[string]interface{}{}
		for _, a := range n.attrs {
			m["@"+a.Name.Local] = a.Value
		}
		m["#text"] = n.text
		return m
	}
	m := map[string]interface{}{}
	for _, a := range n.attrs {
		m["@"+a.Name.Local] = a.Value
	}
	if strings.TrimSpace(n.text) != "" {
		m["#text"] = n.text
	}
	for _, c := range n.children {
		val := nodeToDict(c)
		if existing, ok := m[c.name]; ok {
			switch t := existing.(type) {
			case []interface{}:
				m[c.name] = append(t, val)
			default:
				m[c.name] = []interface{}{existing, val}
			}
		} else {
			m[c.name] = val
		}
	}
	return m
}

func charsetReader(charset string, input io.Reader) (io.Reader, error) {
	// 设备响应基本为 UTF-8；其他编码原样透传
	return input, nil
}

// ConvertToMap 将 *ODict 转普通 map（保序信息丢失）。
func ConvertToMap(d *ODict) map[string]interface{} {
	out := make(map[string]interface{})
	for i, k := range d.keys {
		out[k] = d.vals[i]
	}
	return out
}

// MarshalRequest 将 data（*ODict/map/list/int/string 等）序列化为
// `<request>...</request>`，等价于 Python：
//
//	xmltodict.unparse({"request": data})
//
// 默认输出紧凑格式（无缩进），与 xmltodict.unparse 一致。
func MarshalRequest(data interface{}) ([]byte, error) {
	var buf bytes.Buffer
	buf.WriteString(`<?xml version="1.0" encoding="utf-8"?>`)
	if err := writeElement(&buf, "request", data, 0); err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

// writeElement 递归写出 XML 元素（紧凑模式）。
//
// 值类型规则（等价 xmltodict.unparse）：
//   - string → 文本
//   - *ODict / map → 子元素（map 按键排序回退保序）
//   - []*ODict / []map / []interface{} / []string / []int → 重复同名元素
//   - int / int64 / float64 → 十进制文本
//   - bool → "True" / "False"（Python str(v) 语义）
//   - nil → 空元素 `<x></x>`
func writeElement(buf *bytes.Buffer, name string, val interface{}, depth int) error {
	buf.WriteString("<" + name + ">")

	switch v := val.(type) {
	case nil:
		// 空元素，xmltodict 对 None 输出 <x></x>
	case string:
		buf.WriteString(escapeXML(v))
	case int:
		buf.WriteString(strconv.Itoa(v))
	case int64:
		buf.WriteString(strconv.FormatInt(v, 10))
	case float64:
		buf.WriteString(strconv.FormatFloat(v, 'f', -1, 64))
	case bool:
		if v {
			buf.WriteString("True")
		} else {
			buf.WriteString("False")
		}
	case *ODict:
		for i, k := range v.keys {
			if err := writeElement(buf, k, v.vals[i], depth+1); err != nil {
				return err
			}
		}
	case map[string]interface{}:
		keys := make([]string, 0, len(v))
		for k := range v {
			keys = append(keys, k)
		}
		sort.Strings(keys)
		for _, k := range keys {
			if err := writeElement(buf, k, v[k], depth+1); err != nil {
				return err
			}
		}
	case map[string]string:
		keys := make([]string, 0, len(v))
		for k := range v {
			keys = append(keys, k)
		}
		sort.Strings(keys)
		for _, k := range keys {
			if err := writeElement(buf, k, v[k], depth+1); err != nil {
				return err
			}
		}
	case []interface{}:
		for _, item := range v {
			if err := writeElement(buf, name, item, depth+1); err != nil {
				return err
			}
		}
	case []*ODict:
		for _, item := range v {
			if err := writeElement(buf, name, item, depth+1); err != nil {
				return err
			}
		}
	case []map[string]interface{}:
		for _, item := range v {
			if err := writeElement(buf, name, item, depth+1); err != nil {
				return err
			}
		}
	case []string:
		for _, item := range v {
			if err := writeElement(buf, name, item, depth+1); err != nil {
				return err
			}
		}
	case []int:
		for _, item := range v {
			if err := writeElement(buf, name, item, depth+1); err != nil {
				return err
			}
		}
	default:
		return fmt.Errorf("unsupported xml value type %T for %s", v, name)
	}

	buf.WriteString("</" + name + ">")
	return nil
}

// escapeXML 对 XML 文本做转义（等价 xmltodict 的转义，
// 注意 xmltodict 不转义单引号）。
func escapeXML(s string) string {
	var b strings.Builder
	b.Grow(len(s))
	for _, r := range s {
		switch r {
		case '&':
			b.WriteString("&amp;")
		case '<':
			b.WriteString("&lt;")
		case '>':
			b.WriteString("&gt;")
		case '"':
			b.WriteString("&quot;")
		default:
			b.WriteRune(r)
		}
	}
	return b.String()
}