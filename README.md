# huawei-lte-api-go

华为 LAN/WAN LTE 调制解调器（路由器/上网卡）API 的 **Go 语言移植版**。

本项目是 [Salamek/huawei-lte-api](https://github.com/Salamek/huawei-lte-api)（Python 版 huawei-lte-api **v2.0.1**）的完整 Go 迁移。你可以用它轻松地发送短信、查询流量使用情况、信号强度以及大量其他设备信息。

> 📦 **模块路径**：`github.com/lvcdy/huawei-lte-api-go`（对应远端仓库地址）。

## 特性

- ✅ 完整移植 Python 版全部 API：**45 个 API 分组**（`App`、`Device`、`Monitoring`、`Sms`、`Net`、`WLan`……）、**24 个 config 分组**、**1 个 usermanual 分组**
- ✅ 纯标准库实现，零第三方依赖，Go 1.27+
- ✅ 完整复刻 Python 版行为：CESU-8 编码、XML 序列化、CSRF Token 刷新与重试、RSA 加密（PKCS1_v1_5 / OAEP-SHA1）、JSON 响应解析
- ✅ 自动挂载 CookieJar：与 Python `requests.Session` 一致，会话 Cookie 自动保存与携带（部分新型号如 H168-383 将 CSRF Token 绑定在 SessionID Cookie 上，不带 Cookie 会返回 `125003`；本库对传入的 `http.Client` 自动补 Jar，且不影响全局 `http.DefaultClient`）
- ✅ 与 Python 版一致的错误模型：`ResponseError` 及 14 个派生错误类型（`NotSupportedError`、`LoginRequiredError`、`SystemBusyError`、`LoginCsrfError`……），支持 `errors.Is` / `errors.As` 判断
- ✅ 单元测试覆盖核心逻辑（session、user、cesu8 等），`go test ./...` 全绿

## 测试过的设备

#### 3G/LTE 路由器：
* Huawei B310s-22
* Huawei B311-221
* Huawei B315s-22
* Huawei B525s-23a
* Huawei B525s-65a
* Huawei B715s-23c
* Huawei B528s
* Huawei B535-232
* Huawei B628-265
* Huawei B612-233
* Huawei B818-263
* Huawei E5180s-22
* Huawei E5186s-22a
* Huawei E5576-320
* Huawei E5577Cs-321
* Huawei E8231
* Huawei E5573s-320
* SoyeaLink B535-333

#### 3G/LTE 上网卡（USB 棒）：

（设备必须支持 NETWork 模式，即 "HiLink" 版本，串口模式无法工作）
* Huawei E3131
* Huawei E8372h-608
* Huawei E3372
* Huawei E3531
* Huawei E5530As-2

#### 5G 路由器：

* Huawei 5G CPE Pro 2 (H122-373)
* Huawei 5G CPE Pro (H112-372)
* Huawei 5G CPE6 (H165-383)
* Huawei 5G CPE Ultra 6 (H168-383)（已在真实设备上验证：自动登录、monitoring/status、device/signal、device/information 均正常）

（其他华为 LTE 设备大概率也可用）

### 无法工作：

#### LTE 路由器：
* Huawei B2368-22（固件不兼容，需要测试设备！）
* Huawei B593s-22（固件不兼容，需要测试设备！）

## 项目结构

```
.
├── go/                      # Go 模块根目录（module github.com/lvcdy/huawei-lte-api-go）
│   ├── api/                 # 45 个 API 分组（对应 Python 版 huawei_lte_api/api）
│   ├── config/              # 24 个 config 分组（对应 huawei_lte_api/config）
│   ├── enums/               # 枚举类型（ControlMode、TextMode、Mode……）
│   ├── session/             # 核心会话层：Session、Connection、ApiGroup、
│   │                        #   User/UserSession、CESU-8、XML、RSA、错误类型
│   ├── usermanual/          # 设备使用手册资源接口
│   ├── cmd/                 # 命令行工具（诊断/连通性测试）
│   ├── client.go            # Client 聚合器（对应 Client.py）
│   └── go.mod
├── go.work                  # Go workspace（use ./go）
├── huawei_lte_api/          # 原版 Python 源码（迁移参照，仅作参考）
├── examples/                # Python 版示例脚本
└── pyproject.toml           # Python 版工程文件
```

## 构建与测试

```bash
# 进入 Go 模块目录
cd go

# 构建
go build ./...

# 测试
go test ./...

# 静态检查
go vet ./...
```

> 在仓库根目录构建时使用 `go build all`（由于 `go.work` workspace 模式的限制）。

## 使用方法

与 Python 版用法保持一致：用 `session.NewConnection` 建立连接，用 `NewClient` 聚合各 API 分组。

```go
package main

import (
	"fmt"
	"net/http"
	"time"

	"github.com/lvcdy/huawei-lte-api-go"
	"github.com/lvcdy/huawei-lte-api-go/session"
)

func main() {
	// 在 URL 中内嵌凭据：http://admin:MY_SUPER_TRUPER_PASSWORD@192.168.8.1/
	// 注意：timeout 是 time.Duration，务必使用 5*time.Second 而不是裸数字！
	connection, err := session.NewConnection(
		"http://admin:MY_SUPER_TRUPER_PASSWORD@192.168.8.1/",
		"", "",                // 用户名/密码缺省时从 URL 提取
		5*time.Second,         // 超时
		http.DefaultClient,    // 可传入自定义 *http.Client
	)
	if err != nil {
		panic(err)
	}
	defer connection.Close()

	// 构建 Client，聚合所有 API 分组
	client := huaweilteapi.NewClient(connection)

	// 无需登录即可访问（signal 信号强度）
	signal, err := client.Device.Signal()
	fmt.Println(signal)

	// 需要有效授权，凭据错误会返回错误
	info, err := client.Device.Information()
	fmt.Println(info)
}
```

返回结果为 `map[string]interface{}`，例如 `client.Device.Information()`：

```go
map[string]interface{}{
	"DeviceName":      "B310s-22",
	"SerialNumber":    "MY_SERIAL_NUMBER",
	"Imei":            "MY_IMEI",
	"Imsi":            "MY_IMSI",
	"Iccid":           "MY_ICCID",
	"Msisdn":          nil,
	"HardwareVersion": "WL1B310FM03",
	"SoftwareVersion": "21.311.06.03.55",
	"WebUIVersion":    "17.100.09.00.03",
	"MacAddress1":     "EHM:MY:MAC",
	"MacAddress2":     nil,
	"ProductFamily":   "LTE",
	"Classify":        "cpe",
	"supportmode":     nil,
	"workmode":        "LTE",
}
```

### 发送短信示例

```go
// 导入 enums 包：import "github.com/lvcdy/huawei-lte-api-go/enums"
result, err := client.Sms.SendSms(
	[]string{"+8613800138000"},
	"Hello from Go!",
	0,                       // smsIndex
	nil,                     // sca，可传 *string
	enums.TextModeSevenBit,  // 文本模式（TextModeUCS2/SevenBit/EightBit，见 enums 包）
	nil,                     // fromDate，nil 表示立即发送
)
```

## 代码示例

`examples/` 目录保留了 Python 版示例脚本（可直接对照迁移逻辑），Go 版调用方式可参考 `go/client.go` 与各 API 文件。

更多完整示例：

### 监控

* 流量与信号监控：https://github.com/littlejo/huawei-lte-examples
* B525s-23a 设频段、显示信号等级与带宽：https://github.com/octave21/huawei-lte
* 监控网络连通性、断网自动重启路由器：https://github.com/Salamek/netkeeper
* 带 TUI 界面的监控应用（类似 htop）：https://github.com/pdo-smith/5gtop

### 短信

* 将收到的短信转发到邮箱：https://github.com/chenwei791129/Huawei-LTE-Router-SMS-to-E-mail-Sender

## 其他语言版本

* Python（本项目的原始出处）：https://github.com/Salamek/huawei-lte-api
* TypeScript/JavaScript：https://github.com/Salamek/huawei-lte-api-ts
* PHP：https://github.com/icetee/huawei-lte-api-php

## 捐赠

原项目（Salamek/huawei-lte-api）的捐赠记录：

* 250 CZK (9.79 EUR) 用于 B535-232 设备基金，感谢 @larsvinc！
* 371,69 CZK (14.32 EUR) by Oleg Jusaew
* 292 CZK (11.50 EUR) by Toth-Mate Akos
* 441 CZK (18.12 EUR) by Olexandr Shamin

## 许可证

LGPL-3.0（与上游 Python 版一致）
