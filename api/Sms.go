package api

import (
	"time"

	"github.com/lvcdy/huawei-lte-api-go/enums"
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Message 对应 Sms.py 的 Message dataclass。
type Message struct {
	Index    int             // API 中的索引
	Status   enums.SmsStatus // 短信状态
	Phone    string          // 发送方电话号码
	Content  string          // 短信正文
	DateTime time.Time       // 收发时间
	Sca      *string         // 短信中心号码（INTL 格式，如 +420603052000）
	SaveType enums.SaveMode  // 设备保存方式
	Priority enums.Priority  // 优先级
	Type     enums.SmsType   // 短信类型
	TextMode enums.TextMode  // 编码方式
}

// FromDict 对应 Message.from_dict。
func MessageFromDict(data map[string]interface{}) Message {
	dateStr, _ := session.MapGetString(data, "Date")
	dateTime := time.Now()
	if dateStr != "" {
		if t, err := session.StringToTime(dateStr); err == nil {
			dateTime = t
		}
	}
	return Message{
		Index:    dictInt(data, "Index"),
		Status:   enums.SmsStatus(dictInt(data, "Smstat")),
		Phone:    dictStr(data, "Phone"),
		Content:  dictStr(data, "Content"),
		DateTime: dateTime,
		Sca:      dictStrPtr(data, "Sca"),
		SaveType: enums.SaveMode(dictInt(data, "SaveType")),
		Priority: enums.Priority(dictInt(data, "Priority")),
		Type:     enums.SmsType(dictInt(data, "SmsType")),
	}
}

// ToDict 对应 Message.to_dict。
func (m Message) ToDict() map[string]interface{} {
	sca := interface{}(nil)
	if m.Sca != nil {
		sca = *m.Sca
	}
	return map[string]interface{}{
		"Index":    itoa(m.Index),
		"Smstat":   itoa(int(m.Status)),
		"Phone":    m.Phone,
		"Content":  m.Content,
		"Date":     session.TimeToString(m.DateTime),
		"Sca":      sca,
		"SaveType": itoa(int(m.SaveType)),
		"Priority": itoa(int(m.Priority)),
		"SmsType":  itoa(int(m.Type)),
	}
}

// Sms 对应 Sms.py。
type Sms struct {
	*session.ApiGroup
}

// NewSms 创建 Sms API 分组。
func NewSms(s *session.Session) *Sms {
	return &Sms{ApiGroup: session.NewApiGroup(s)}
}

// GetCbsnewslist 对应 sms/get-cbsnewslist。
func (s *Sms) GetCbsnewslist() (map[string]interface{}, error) {
	return s.S.Get("sms/get-cbsnewslist", nil, "api")
}

// SmsCount 对应 sms/sms-count。
func (s *Sms) SmsCount() (map[string]interface{}, error) {
	return s.S.Get("sms/sms-count", nil, "api")
}

// SplitinfoSms 对应 sms/splitinfo-sms。
func (s *Sms) SplitinfoSms() (map[string]interface{}, error) {
	return s.S.Get("sms/splitinfo-sms", nil, "api")
}

// SmsFeatureSwitch 对应 sms/sms-feature-switch。
func (s *Sms) SmsFeatureSwitch() (map[string]interface{}, error) {
	return s.S.Get("sms/sms-feature-switch", nil, "api")
}

// SendStatus 对应 sms/send-status。
func (s *Sms) SendStatus() (map[string]interface{}, error) {
	return s.S.Get("sms/send-status", nil, "api")
}

// GetSmsList 获取短信列表。对应 sms/sms-list (post_get) + enforce_list_response。
// 注意：至少 B525s-23a 对字段顺序敏感，使用保序 ODict。
func (s *Sms) GetSmsList(
	page int,
	boxType enums.BoxType,
	readCount int,
	sortType enums.SortType,
	ascending bool,
	unreadPreferred bool,
) (map[string]interface{}, error) {
	res, err := s.S.PostGet("sms/sms-list", session.O(
		"PageIndex", page,
		"ReadCount", readCount,
		"BoxType", int(boxType),
		"SortType", int(sortType),
		"Ascending", boolToInt(ascending),
		"UnreadPreferred", boolToInt(unreadPreferred),
	), false, "api", false, false)
	if err != nil {
		return nil, err
	}
	plural := "Messages"
	return session.EnforceListResponse(res, "Message", &plural), nil
}

// DeleteSms 按 ID 删除单条短信。对应 sms/delete-sms (post_set)。
func (s *Sms) DeleteSms(smsID int) (interface{}, error) {
	return s.S.PostSet("sms/delete-sms", map[string]interface{}{"Index": smsID}, false, "api", false, false)
}

// BackupSim 备份短信到 SIM 卡。对应 sms/backup-sim (post_set)。
func (s *Sms) BackupSim(fromDate time.Time, isMove bool) (interface{}, error) {
	return s.S.PostSet("sms/backup-sim", session.O(
		"IsMove", boolToInt(isMove),
		"Date", session.TimeToString(fromDate),
	), false, "api", false, false)
}

// SetRead 标记短信为已读。对应 sms/set-read (post_set)。
func (s *Sms) SetRead(smsID int) (interface{}, error) {
	return s.S.PostSet("sms/set-read", map[string]interface{}{"Index": smsID}, false, "api", false, false)
}

// SaveSms 保存短信。对应 sms/save-sms (post_set)。
func (s *Sms) SaveSms(
	phoneNumbers []string,
	message string,
	smsIndex int,
	sca *string,
	textMode enums.TextMode,
	fromDate *time.Time,
) (interface{}, error) {
	sendDate := time.Now().UTC()
	if fromDate != nil {
		sendDate = *fromDate
	}
	return s.S.PostSet("sms/save-sms", session.O(
		"Index", smsIndex,
		"Phones", map[string]interface{}{"Phone": phoneNumbers},
		"Sca", ptrOrNil(sca),
		"Content", message,
		"Length", len(message),
		"Reserved", int(textMode),
		"Date", session.TimeToString(sendDate),
	), false, "api", false, false)
}

// SendSms 发送短信。对应 sms/send-sms (post_set)。
func (s *Sms) SendSms(
	phoneNumbers []string,
	message string,
	smsIndex int,
	sca *string,
	textMode enums.TextMode,
	fromDate *time.Time,
) (interface{}, error) {
	sendDate := time.Now().UTC()
	if fromDate != nil {
		sendDate = *fromDate
	}
	return s.S.PostSet("sms/send-sms", session.O(
		"Index", smsIndex,
		"Phones", map[string]interface{}{"Phone": phoneNumbers},
		"Sca", ptrOrNil(sca),
		"Content", message,
		"Length", len(message),
		"Reserved", int(textMode),
		"Date", session.TimeToString(sendDate),
	), false, "api", false, false)
}

// CancelSend 取消发送。对应 sms/cancel-send (post_set)，传 int 1。
func (s *Sms) CancelSend() (interface{}, error) {
	return s.S.PostSet("sms/cancel-send", 1, false, "api", false, false)
}

// Config 对应 sms/config。
func (s *Sms) Config() (map[string]interface{}, error) {
	return s.S.Get("sms/config", nil, "api")
}

// SetConfig 设置默认短信发送配置。对应 sms/config (post_set)。
func (s *Sms) SetConfig(
	sca string,
	saveMode enums.SaveMode,
	validity int,
	useSReport bool,
	sendType enums.SendType,
	priority enums.Priority,
) (interface{}, error) {
	return s.S.PostSet("sms/config", session.O(
		"SaveMode", int(saveMode),
		"Validity", validity,
		"Sca", sca,
		"UseSReport", useSReport,
		"SendType", int(sendType),
		"Priority", int(priority),
	), false, "api", false, false)
}

// SmsCountContact 对应 sms/sms-count-contact。
func (s *Sms) SmsCountContact() (map[string]interface{}, error) {
	return s.S.Get("sms/sms-count-contact", nil, "api")
}

// SmsListContact 对应 sms/sms-list-contact (post_get)。
func (s *Sms) SmsListContact(pageindex int, readcount int) (map[string]interface{}, error) {
	return s.S.PostGet("sms/sms-list-contact", map[string]interface{}{
		"pageindex": pageindex,
		"readcount": readcount,
	}, false, "api", false, false)
}

// GetSmsListPdu 以 PDU 格式返回短信。对应 sms/sms-list-pdu (post_get)。
func (s *Sms) GetSmsListPdu(
	page int,
	boxType enums.BoxType,
	readCount int,
) (map[string]interface{}, error) {
	return s.S.PostGet("sms/sms-list-pdu", map[string]interface{}{
		"PageIndex": page,
		"ReadCount": readCount,
		"BoxType":   int(boxType),
	}, false, "api", false, false)
}

// SplitSms 对应 sms/split-sms。
func (s *Sms) SplitSms() (map[string]interface{}, error) {
	return s.S.Get("sms/split-sms", nil, "api")
}

// SendSmsPdu 发送 PDU 短信。对应 sms/send-sms-pdu (post_set)。
func (s *Sms) SendSmsPdu(
	pdu string,
	length int,
	smsIndex int,
	sca *string,
	validity int,
	statusReport bool,
	saveMode enums.SaveMode,
	sendType enums.SendType,
) (interface{}, error) {
	return s.S.PostSet("sms/send-sms-pdu", map[string]interface{}{
		"Index":      smsIndex,
		"PDU":        pdu,
		"Length":     length,
		"SaveMode":   int(saveMode),
		"Validity":   validity,
		"Sca":        ptrOrNil(sca),
		"UseSReport": boolToInt(statusReport),
		"SendType":   int(sendType),
	}, false, "api", false, false)
}

// RecoverSms 对应 sms/recover-sms。
func (s *Sms) RecoverSms() (map[string]interface{}, error) {
	return s.S.Get("sms/recover-sms", nil, "api")
}

// CopySms 对应 sms/copy-sms。
func (s *Sms) CopySms() (map[string]interface{}, error) {
	return s.S.Get("sms/copy-sms", nil, "api")
}

// MoveSms 对应 sms/move-sms。
func (s *Sms) MoveSms() (map[string]interface{}, error) {
	return s.S.Get("sms/move-sms", nil, "api")
}

// GetMessages 迭代获取所有短信。对应 Sms.get_messages 生成器。
// 若 readCount<=0 则按 20 条分块读取全部；否则单页读取。
// 返回消息列表即可（跳过 60 秒内新到的可能分片短信，保持原语义）。
func (s *Sms) GetMessages(
	page int,
	boxType enums.BoxType,
	readCount int,
	sortType enums.SortType,
	ascending bool,
	unreadPreferred bool,
) ([]Message, error) {
	// 未指定读取条数：分块读取全部。
	if readCount <= 0 {
		readCount = 20
		page = 1
	}

	var result []Message
	now := time.Now()
	for {
		smsListTmp, err := s.GetSmsList(page, boxType, readCount, sortType, ascending, unreadPreferred)
		if err != nil {
			return nil, err
		}
		if dictInt(smsListTmp, "Count") == 0 {
			break
		}

		messages := toDict(smsListTmp["Messages"])
		for _, messageRaw := range toSlice(messages["Message"]) {
			md, ok := messageRaw.(map[string]interface{})
			if !ok {
				continue
			}
			message := MessageFromDict(md)
			// 若为可能的多部分短信，且比 60 秒内新到，跳过
			// （给路由器足够时间接收分片并重建）。
			if message.Type == enums.SmsTypeMultipart && message.DateTime.Add(10*time.Second).After(now) {
				continue
			}
			result = append(result, message)
		}
		page++
	}
	return result, nil
}
