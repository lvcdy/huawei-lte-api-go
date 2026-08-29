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
- ✅ 单元测试覆盖核心逻辑（session、user、cesu8 等）及本库新增的 5G 专有端点，`go test ./...` 全绿
- ✅ **5G CPE 专有端点**（参考 Brovi-Huawei-5G-CPE-Manager 补齐）：`device/seccellinfo`（载波聚合辅小区）、`device/nbrcellinfo`（邻小区）、`net/antenna-configuration`（天线配置）、`net/lock-cell`（锁频）、`developermode/developer-mode|developer-item`（开发者模式）、`app/atport-status`（AT 调试端口）、`wlan/wlan-debug`（WLAN 调试）
- ✅ **设备实测工具** `cmd/cpetest`：一条命令遍历上述 5G 端点，读操作默认全跑、写操作需显式 flag

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
* Huawei 5G CPE Ultra 6 (H168-383)

（其他华为 LTE 设备大概率也可用）

### 无法工作：

#### LTE 路由器：
* Huawei B2368-22（固件不兼容，需要测试设备！）
* Huawei B593s-22（固件不兼容，需要测试设备！）

## 项目结构

```
.
├── api/                    # 45 个 API 分组（对应 Python 版 huawei_lte_api/api）
├── config/                 # 24 个 config 分组（对应 huawei_lte_api/config）
├── enums/                  # 枚举类型（ControlMode、TextMode、Mode……）
├── session/                # 核心会话层：Session、Connection、ApiGroup、
│                           #   User/UserSession、CESU-8、XML、RSA、错误类型
├── usermanual/             # 设备使用手册资源接口
├── cmd/                    # 命令行工具：
│   ├── cpecheck/           #   连通性/信号测试（诊断）
│   ├── cpedebug/           #   原始 HTTP 登录流程调试
│   └── cpetest/            #   5G 专有端点实测（seccellinfo/lock-cell 等）
├── client.go               # Client 聚合器（对应 Client.py）
├── go.mod                  # Go 模块定义（module github.com/lvcdy/huawei-lte-api-go）
├── .github/workflows/      # GitHub Actions 工作流
└── README.md
```

## 构建与测试

```bash
# 构建
go build ./...

# 测试
go test ./...

# 静态检查
go vet ./...
```

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

## 5G CPE 专有端点

以下端点为本库参考 Android 应用 [Brovi-Huawei-5G-CPE-Manager](https://github.com/fz911a/Brovi-Huawei-5G-CPE-Manager) 补充，主要面向 5G CPE（H122-373 / H168-383 等）。部分端点需要**开发者模式登录**（loginflag=2 挑战认证），普通 admin 凭据可能返回 `PermissionDeniedError`。

| Go 方法 | HTTP 端点 | 说明 | 需要开发者模式 |
| --- | --- | --- | --- |
| `Device.SecCellInfo()` | `GET device/seccellinfo` | 载波聚合**辅小区**（SCell）信息，返回 `ARFCN,Band,PCI,RSRP,...;...` 风格 CSV | 部分固件 |
| `Device.NbrCellInfo()` | `GET device/nbrcellinfo` | **邻小区**信息，返回 `ARFCN,Band,PCI,RSRP,...;...` 风格 CSV | 部分固件 |
| `Net.AntennaConfiguration()` | `GET net/antenna-configuration` | 天线配置（模式/增益） | 否 |
| `Ntwk.LockCell(lock, freq, pci)` | `POST net/lock-cell` | 锁定/解锁指定频点小区（`lock=1` 锁、`0` 解，`freq`/`pci` 传 0 表示清除） | 是 |
| `Developer.DeveloperMode()` | `GET developermode/developer-mode` | 开发者模式开关状态 | 是 |
| `Developer.DeveloperItem()` | `GET developermode/developer-item` | 开发者模式子项（telnet 等） | 是 |
| `Developer.AtportStatus()` | `GET app/atport-status` | AT 调试端口状态查询 | 否（写需要） |
| `Developer.SetAtportStatus(enable)` | `POST app/atport-status` | 开启/关闭 AT 调试端口（如 Telnet 20249） | 是 |
| `WLan.WlanDebug()` | `GET wlan/wlan-debug` | WLAN 调试配置查询 | 是 |
| `WLan.SetWlanDebug(fields)` | `POST wlan/wlan-debug` | 写入 WLAN 调试字段（键名随固件） | 是 |

```go
// 载波聚合信息（5G CPE）
sec, err := client.Device.SecCellInfo()
fmt.Println(sec["nrseccell_list"]) // "123,78,0,501,440,-80,-95,-11,8;..."

nbr, err := client.Device.NbrCellInfo()
fmt.Println(nbr["nbrcell_ltelist"])

// 天线配置（无需开发者模式）
ant, err := client.Net.AntennaConfiguration()
fmt.Println(ant) // map[gain:"3.5" ...]

// ---- 以下需要开发者模式登录 ----

// 锁定 频点 1450 / PCI 501 的小区
resp, err := client.Ntwk.LockCell(1, 1450, 501)
// 解锁（复位到自动选网）
resp, err = client.Ntwk.LockCell(0, 0, 0)

// 查询/开关 AT 调试端口
status, _ := client.Developer.AtportStatus()
_, err = client.Developer.SetAtportStatus(1) // 开
_, err = client.Developer.SetAtportStatus(0) // 关

// WLAN 调试配置
debug, _ := client.WLan.WlanDebug()
_, err = client.WLan.SetWlanDebug(map[string]interface{}{
	"telnet_enable": 0,
})
```

> ⚠️ **锁频与开发者模式写入会改变设备工作状态**，请确认目标频点/PCI 后再执行，并及时用 `LockCell(0,0,0)` / `SetAtportStatus(0)` 复位。

## 命令行工具

仓库 `cmd/` 下提供三个开箱即用的工具：

### cpecheck —— 连通性与信号测试

```bash
go run ./cmd/cpecheck --url=http://192.168.8.1/ --username=admin --password=xxxx
# 连带输出 monitoring/status、device/signal、device/information 结果
```

### cpedebug —— 原始 HTTP 登录流程调试

用于排查 handshake/challenge 阶段的异常，输出原始请求响应。

### cpetest —— 5G 专有端点实测

```bash
# 只读端点全部执行（默认）
go run ./cmd/cpetest --url=http://192.168.8.1/ --username=admin --password=xxxx

# 跳过部分端点
# go run ./cmd/cpetest -skip seccellinfo,nbrcellinfo ...

# 写端点（须显式指定；下述命令执行后再见 README 提醒复位）
# 锁定小区 频点1450/PCI501
# go run ./cmd/cpetest -set-lock-cell 1450:501 ...
# 清除锁频
# go run ./cmd/cpetest -set-lock-cell 0:0 ...
# 开启 AT 调试端口（Telnet）
# go run ./cmd/cpetest -set-atport 1 ...
# 写入 wlan 调试字段
# go run ./cmd/cpetest -set-wlan-debug telnet_enable=0,developermode_enable=1 ...
```

## 更多示例

Go 版调用方式可参考 `client.go` 与各 API 文件；完整示例仓库如下：

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
