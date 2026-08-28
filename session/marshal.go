package session

import "encoding/json"

// CreateRequestXML 对应 Python 版 Session._create_request_xml：
//
//	wrapped_in_request = {"request": data}
//	return cesu8_encode(xmltodict.unparse(wrapped_in_request))
func CreateRequestXML(data interface{}) ([]byte, error) {
	xmlBytes, err := MarshalRequest(data)
	if err != nil {
		return nil, err
	}
	return Cesu8Encode(string(xmlBytes))
}

// jsonMarshal 序列化 JSON（标准库，无 HTML 转义问题用默认即可）。
func jsonMarshal(v interface{}) ([]byte, error) {
	return json.Marshal(v)
}
