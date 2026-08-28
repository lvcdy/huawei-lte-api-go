package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Voice 对应 Voice.py。
type Voice struct {
	*session.ApiGroup
}

// NewVoice 创建 Voice API 分组。
func NewVoice(s *session.Session) *Voice {
	return &Voice{ApiGroup: session.NewApiGroup(s)}
}

// Featureswitch 对应 voice/featureswitch。
func (v *Voice) Featureswitch() (map[string]interface{}, error) {
	return v.S.Get("voice/featureswitch", nil, "api")
}

// Sipaccount 对应 voice/sipaccount。
func (v *Voice) Sipaccount() (map[string]interface{}, error) {
	return v.S.Get("voice/sipaccount", nil, "api")
}

// Sipadvance 对应 voice/sipadvance。
func (v *Voice) Sipadvance() (map[string]interface{}, error) {
	return v.S.Get("voice/sipadvance", nil, "api")
}

// Sipserver 对应 voice/sipserver。
func (v *Voice) Sipserver() (map[string]interface{}, error) {
	return v.S.Get("voice/sipserver", nil, "api")
}

// Speeddial 对应 voice/speeddial。
func (v *Voice) Speeddial() (map[string]interface{}, error) {
	return v.S.Get("voice/speeddial", nil, "api")
}

// Functioncode 对应 voice/functioncode。
func (v *Voice) Functioncode() (map[string]interface{}, error) {
	return v.S.Get("voice/functioncode", nil, "api")
}

// Voiceadvance 对应 voice/voiceadvance。
func (v *Voice) Voiceadvance() (map[string]interface{}, error) {
	return v.S.Get("voice/voiceadvance", nil, "api")
}

// Voicebusy 对应 voice/voicebusy。
func (v *Voice) Voicebusy() (map[string]interface{}, error) {
	return v.S.Get("voice/voicebusy", nil, "api")
}

// Codec 对应 voice/codec。
func (v *Voice) Codec() (map[string]interface{}, error) {
	return v.S.Get("voice/codec", nil, "api")
}

// Voiperstatus 对应 voice/voiperstatus。
func (v *Voice) Voiperstatus() (map[string]interface{}, error) {
	return v.S.Get("voice/voiperstatus", nil, "api")
}

// Volte 对应 voice/volte。
func (v *Voice) Volte() (map[string]interface{}, error) {
	return v.S.Get("voice/volte", nil, "api")
}