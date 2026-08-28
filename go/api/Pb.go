package api

import (
	"github.com/lvcdy/huawei-lte-api-go/session"
)

// Pb 对应 Pb.py。
type Pb struct {
	*session.ApiGroup
}

// NewPb 创建 Pb API 分组。
func NewPb(s *session.Session) *Pb {
	return &Pb{ApiGroup: session.NewApiGroup(s)}
}

// GetPbMatch 在电话簿中查找号码。对应 pb/pb-match (post_get)。
func (p *Pb) GetPbMatch(phoneNumber string) (map[string]interface{}, error) {
	return p.S.PostGet("pb/pb-match", map[string]interface{}{
		"Phone": phoneNumber,
	}, false, "api", false, false)
}

// GetPbList 获取电话簿条目列表。对应 pb/pb-list (post_get)。
func (p *Pb) GetPbList(
	page int,
	keyWord string,
	groupId int,
	readCount int,
	saveType int,
	sortType int,
	ascending int,
) (map[string]interface{}, error) {
	return p.S.PostGet("pb/pb-list", session.O(
		"GroupID", groupId,
		"PageIndex", page,
		"ReadCount", readCount,
		"SaveType", saveType,
		"SortType", sortType,
		"Ascending", ascending,
		"KeyWord", keyWord,
	), false, "api", false, false)
}

// PbCount 获取电话簿条目数量。对应 pb/pb-count (post_get)。
func (p *Pb) PbCount() (map[string]interface{}, error) {
	return p.S.PostGet("pb/pb-count", nil, false, "api", false, false)
}

// GroupCount 获取电话簿分组数量。对应 pb/group-count (post_get)。
func (p *Pb) GroupCount() (map[string]interface{}, error) {
	return p.S.PostGet("pb/group-count", nil, false, "api", false, false)
}

// pbField 电话簿联系人字段节点，对应 Python 的 Node("Field")。
// 注意 Python 中每个 Node 实例都是独立 key，因此 5 个 Field 全部序列化。
func pbField(name, value string) *session.ODict {
	return session.O("Name", name, "Value", value)
}

// PbNew 添加新的电话簿条目。对应 pb/pb-new (post_set)。
//
// 注意：Python 版使用 5 个独立的 Node("Field") key，Go 版用 []*ODict
// 输出重复的 <Field> 节点，行为一致。
func (p *Pb) PbNew(
	groupId int,
	saveType int,
	name string,
	mobilePhone string,
	homePhone string,
	workPhone string,
	workEmail string,
) (interface{}, error) {
	return p.S.PostSet("pb/pb-new", session.O(
		"GroupID", groupId,
		"SaveType", saveType,
		"Field", []*session.ODict{
			pbField("FormattedName", name),
			pbField("MobilePhone", mobilePhone),
			pbField("HomePhone", homePhone),
			pbField("WorkPhone", workPhone),
			pbField("WorkEmail", workEmail),
		},
	), false, "api", false, false)
}

// PbDelete 按 ID 删除电话簿条目。对应 pb/pb-delete (post_set)。
func (p *Pb) PbDelete(pbIndex int) (interface{}, error) {
	return p.S.PostSet("pb/pb-delete", map[string]interface{}{
		"Index": pbIndex,
	}, false, "api", false, false)
}

// GroupDelete 按 ID 删除电话簿分组。对应 pb/group-delete (post_set)。
func (p *Pb) GroupDelete(groupId int) (interface{}, error) {
	return p.S.PostSet("pb/group-delete", map[string]interface{}{
		"GroupID": groupId,
	}, false, "api", false, false)
}

// GroupList 获取电话簿分组列表。对应 pb/group-list (post_get)。
func (p *Pb) GroupList(
	page int,
	readCount int,
	sortType int,
	ascending int,
) (map[string]interface{}, error) {
	return p.S.PostGet("pb/group-list", session.O(
		"PageIndex", page,
		"ReadCount", readCount,
		"SortType", sortType,
		"Ascending", ascending,
	), false, "api", false, false)
}

// GroupNew 按名称创建电话簿分组。对应 pb/group-new (post_set)。
func (p *Pb) GroupNew(nameStr string) (interface{}, error) {
	return p.S.PostSet("pb/group-new", map[string]interface{}{
		"GroupName": nameStr,
	}, false, "api", false, false)
}