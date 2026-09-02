# huawei-lte-api

华为 LAN/WAN LTE/5G 调制解调器（路由器/上网卡）API 的 **Rust 版**。

本项目是 [Salamek/huawei-lte-api](https://github.com/Salamek/huawei-lte-api)（Python 版 huawei-lte-api **v2.0.1**）的完整 Rust 迁移，并结合 [Brovi-Huawei-5G-CPE-Manager](https://github.com/fz911a/Brovi-Huawei-5G-CPE-Manager) 补入了 5G CPE 专有端点。仓库根目录（原为 Go 语言版本）已升级为可直接 `cargo build` / `cargo test` 的 Cargo 项目。你可以用它轻松地发送短信、查询流量使用情况、信号强度以及大量其他设备信息。

## 特性

- ✅ 完整移植 Python 版全部端点 + Brovi 5G 补充端点：**45 个 API 分组**（`App`、`Device`、`Monitoring`、`Sms`、`Net`、`WLan`…… 及 Brovi 新增的 `Developermode`）、**`User` 认证分组**、**24 个 config 分组**、**1 个 usermanual 分组**
- ✅ 忠实复刻 Python 版协议行为：CESU-8 编码、XML 序列化、`__RequestVerificationToken` CSRF 令牌刷新与重试、XML/JSON 响应解析、错误码 → 强类型 `Error` 映射
- ✅ 内置 `reqwest`（blocking）cookie 会话，行为与 Python `requests.Session` 一致
- ✅ 与 Python 版一致的错误体系：`Error` 统一错误类型，涵盖 `NotSupported` / `LoginRequired` / `SystemBusy` / `LoginCsrf` / 各种登录错误码等
- ✅ GET 端点返回 `serde_json::Value`（设备原始响应），POST/操作端点返回 `Result<String>`（通常为 `"OK"`）
- ✅ 5G CPE 专有端点（参考 Brovi-Huawei-5G-CPE-Manager 补入）：
  - `Device`：`device/seccellinfo`（辅小区/载波聚合）、`device/nbrcellinfo`（邻区）
  - `Net`：`net/antenna-configuration`、`net/lock-cell`（按 ARFCN + PCI 锁定小区）
  - `Developermode`（新分组）：`developermode/developer-mode`、`developermode/developer-item`
  - `Developer`：`developer/atport-status`（POST，开关 AT/telnet 调试口）
  - `WLan`：`wlan/wlan-debug`（GET/POST，任意可写调试字段）
- ✅ **170+ 个 mock 单元测试全绿**：通过 `HttpTransport` trait 注入 `FakeTransport`，覆盖 session/xml/枚举逻辑 + 全部 45 个 API 分组 + User + 24 个 config 分组（无需真实设备）

## 快速开始

```toml
[dependencies]
huawei-lte-api = { git = "https://github.com/lvcdy/huawei-lte-api-go" }
```

```rust,no_run
use huawei_lte_api::{Client, Connection};

fn main() -> Result<(), huawei_lte_api::Error> {
    let connection = Connection::new("http://192.168.8.1/", None, None)?;
    // 默认用户名 admin；显式登录：
    connection.login("admin", "password")?;

    let client = Client::new(&connection);

    // 设备信息
    let info = client.device().information()?;
    println!("Device: {info}");

    // 信号/状态
    let status = client.monitoring().status()?;
    println!("Status: {status}");

    // config 分组
    let wifi = client.config_wifi().config()?;
    println!("WLAN config: {wifi}");

    Ok(())
}
```

> 各 `Value` 返回的是设备返回的 JSON 翻译结果，字段名沿用 CPE 的大写名（如 `DeviceName`、`IMEI`），可直接按下标访问：`value["DeviceName"]`。

## 环境要求

- Rust 2021 edition（稳定版工具链即可）
- 依赖：`reqwest`、`quick-xml`、`serde`/`serde_json`、`thiserror`、`base64`、`rsa`、`sha2`、`chrono` 等（见 [`Cargo.toml`](Cargo.toml)）

## 构建与测试

```bash
cargo build --all-targets
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
cargo doc --no-deps
```

## 真实设备测试

仓库内置了一个对真实 CPE 的**只读连通性测试**（不修改任何设备配置）：

```bash
# 无登录测试（公开端点：device/basic_information、monitoring/status、net/current-plmn…）
cargo run --example live_test

# 带登录测试（需正确密码，通过环境变量传入）
$env:HUAWEI_USERNAME="admin"; $env:HUAWEI_PASSWORD="你的密码"; cargo run --example live_test

# 每设备流量统计（登录后拉取 system/HostInfo，按总流量排序打印表格）
$env:HUAWEI_USERNAME="admin"; $env:HUAWEI_PASSWORD="你的密码"; cargo run --example device_traffic

# 端点扫描（登录后对约 90 个候选端点逐个 GET，标记可用的 DATA 端点）
$env:HUAWEI_USERNAME="admin"; $env:HUAWEI_PASSWORD="你的密码"; cargo run --example scan_endpoints
```

> ⚠️ Windows 终端默认 GBK 代码页会把输出中的中文显示成乱码（如 `涓浗绉诲姩`）。
> 这只是**终端显示问题，不是库 bug**——设备返回的是合法 UTF-8（`中国移动`）。验证时请先执行：
> ```powershell
> [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
> ```
> 该行为已由回归测试 `cesu8_fix_preserves_normal_utf8_chinese` 固化（真实设备 `api/net/current-plmn` 的 `中国移动` 字节不被 CESU-8 修复误伤）。

在 H168-383 设备上实测通过的端点：

- **公开（未登录）**：`device/basic_information`、`monitoring/status`、`monitoring/check-notifications`、`user/state-login`、`net/current-plmn`（`中国移动`）；受保护端点（`device/information`、`device/signal`）在未登录时正确返回 `No rights (needs login)`。
- **认证后（登录成功）**：
  - **每设备流量/主机列表**：`system/HostInfo`（JSON 数组，每设备含 `TxKBytes`/`RxKBytes`/`UpRate`/`DownRate`，见 `examples/device_traffic.rs`）、`wlan/host-list`（当前在线 WiFi 主机）、`lan/HostInfo`（DHCP 主机全列表）。
  - **monitoring 组**：`monitoring/status`、`traffic-statistics`、`month_statistics`、`start_date`、`converged-status`、`check-notifications`、`daily-data-limit`、`statistic-feature-switch`、`onekey_diag`。
  - **system 组**：`system/deviceinfoex`、`system/devcapacity`、`system/onlinestate`（含最新升级日志、设备名/序列号/固件版本）。
  - **wlan 组**：`wlan/multi-basic-settings`（SSID/加密模式列表）、`multi-security-settings`、`multi-security-settings-ex`、`multi-switch-settings`、`multi-macfilter-settings`、`multi-macfilter-settings-ex`、`wifi-feature-switch`。
  - **net 组**：`net/net-mode`、`net/network`、`net/net-mode-list`（可用制式/频段列表）、`net/register`、`net/cell-info`（小区 ID + LAC）。
  - **security 组**：`security/mac-filter`、`security/upnp`。
  - **dhcp 组**：`dhcp/settings`（网段/租期/DNS）。
  - **sms 组**：`sms/sms-count`。
  - 其余：`user/login`（SHA256 + CSRF + RSA）、`device/information`、`device/boot_time`、`device/antenna_type`、`device/antenna_set_type`、`user/logout`。
- **设备固件权限限制**（返回 `100003`，需更高权限账号，属正常设备行为，非库 bug）：`device/antenna_status`、`device/antenna_settings`、`wlan/basic_settings`。诊断工具 `examples/diag_endpoints.rs` 可登录后 dump 各端点的原始 XML 响应以区分设备行为与库解析问题。

## 目录结构

```
src/
  session.rs       # 协议请求核心（CSRF、CESU-8、XML/JSON、错误映射、重试）
  connection.rs    # Connection = Session + 当前登录用户
  client.rs        # 顶层聚合：暴露全部 45 + 24 + 1 个分组 getter
  user.rs          # User 认证分组（登录、登出、状态等）
  xml.rs           # XML ↔ XmlMap ↔ serde_json::Value
  errors.rs        # Error 错误类型 + 错误码
  tools.rs         # 工具函数（RSA 加密、map 构造、时间换算等）
  enums/           # 强类型枚举（NetworkMode、AntennaType、PasswordType……）
  api/             # 45 个 API 分组（含 Brovi 新增 Developermode）
  config/          # 24 个 config 分组
  usermanual/      # 1 个 usermanual 分组
  testsupport.rs   # 测试专用 mock transport（不参与生产构建）
```

## 设计说明

- **GET / 查询端点** → `Result<serde_json::Value>`：设备响应 `XmlMap` → JSON 的透明翻译，避免对 CPE 响应 schema 过度建模。
- **POST / 操作端点** → `Result<String>`：响应文本（CPE 通常回 `OK`）。
- **登录**：`User` 分组忠实实现 `_encode_password`（默认 BASE64，设备要求 SHA256 时走 SHA256 派生），并在成功后标记会话已认证、记录 `UserSession`。
- **错误处理**：`session.rs` 依据设备错误码（如 `108001`/`108002`/`125002`…）映射到 `errors::Error` 的类型化变体（`UsernameWrong`/`PasswordWrong`/`LoginCsrf`…），并对 CSRF 失效自动 reload 重试一次。

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

#### 5G 路由器：
* Huawei H832-2
* Huawei H5787-1w
* Huawei H122-373
* Huawei H168-383 (1456)
* Huawei 5G CPE Pro H312-381

## 许可

MIT — 见 [LICENSE](LICENSE)。