// cpetest 针对 5G CPE 专有端点进行设备实测（补齐自 Brovi 的端点）。
//
// 用法：
//
//	go run ./cmd/cpetest [--url=http://192.168.8.1/] [--username=admin] [--password=xxxx]
//
// 只读端点（默认全部执行，可 -skip 跳过）：
//   - device/seccellinfo              （载波聚合辅小区）
//   - device/nbrcellinfo              （邻小区）
//   - net/antenna-configuration       （天线配置）
//   - developermode/developer-mode    （开发者模式开关）
//   - developermode/developer-item    （开发者子项）
//   - app/atport-status               （AT 端口状态查询）
//   - wlan/wlan-debug                 （WLAN 调试配置查询）
//
// 写端点（默认不执行，须显式开启，修改设备状态后再跑一次复位）：
//   - --set-lock-cell=FREQ:PCI        锁定小区（如 1450:501），0:0 清除
//   - --set-atport=1|0                开启/关闭 AT 调试端口（Telnet）
//   - --set-wlan-debug=k=v,k2=v2      写入 WLAN 调试字段
//
// 注意：developermode / wlan-debug / atport-status 写入大多要求
// 开发者模式登录（loginflag=2 挑战认证），普通 admin 登录可能返回
// PermissionDeniedError。此时可先尝试任意密码登录（设备对部分 GET
// 端点放开挑战），或使用开发者模式凭据。
package main

import (
	"flag"
	"fmt"
	"net/http"
	"os"
	"strings"
	"time"

	huaweilteapi "github.com/lvcdy/huawei-lte-api-go"
	"github.com/lvcdy/huawei-lte-api-go/session"
)

func main() {
	var rawURL, username, password, skip, setLockCell, setAtport, setWlanDebug string
	flag.StringVar(&rawURL, "url", "http://192.168.8.1/", "CPE 设备地址")
	flag.StringVar(&username, "username", "", "登录用户名（缺省走匿名/URL 内嵌）")
	flag.StringVar(&password, "password", "", "登录密码")
	flag.StringVar(&skip, "skip", "", "逗号分隔要跳过的只读端点名（如 seccellinfo,nbrcellinfo）")
	flag.StringVar(&setLockCell, "set-lock-cell", "", "写入锁频 FREQ:PCI（如 1450:501；0:0 清除），缺省不写入")
	flag.StringVar(&setAtport, "set-atport", "", "写入 AT 端口开关：1 开 / 0 关，缺省不写入")
	flag.StringVar(&setWlanDebug, "set-wlan-debug", "", "写入 WLAN 调试字段 k=v,k2=v2，缺省不写入")
	flag.Parse()

	if username != "" || password != "" {
		fmt.Println("⚠️  部分端点仅支持开发者模式登录（loginflag=2），普通凭据可能失败")
	}

	// 登录（凭据存在时自动登录；否则匿名）。
	var c interface{ Close() }
	var client *huaweilteapi.Client
	if username != "" || password != "" {
		conn, err := session.NewConnection(rawURL, username, password, 10*time.Second, &http.Client{})
		if err != nil {
			fatal("连接失败: %v", err)
		}
		c = conn
		client = huaweilteapi.NewClient(conn)
	} else {
		s, err := session.NewSession(rawURL, 10*time.Second, &http.Client{})
		if err != nil {
			fatal("连接失败: %v", err)
		}
		c = s
		client = huaweilteapi.NewClient(&session.Connection{Session: s})
	}
	defer c.Close()

	skipSet := map[string]bool{}
	for _, name := range strings.Split(skip, ",") {
		if name = strings.TrimSpace(name); name != "" {
			skipSet[name] = true
		}
	}

	// ---- 只读端点 ----
	fmt.Println("── 只读端点 ──────────────────────────────")
	get := func(name, endpoint string, fn func() (map[string]interface{}, error)) {
		if skipSet[name] {
			fmt.Printf("  %-22s ⏭ 跳过\n", endpoint)
			return
		}
		data, err := fn()
		report(name, endpoint, data, err)
	}
	get("seccellinfo", "device/seccellinfo", client.Device.SecCellInfo)
	get("nbrcellinfo", "device/nbrcellinfo", client.Device.NbrCellInfo)
	get("antenna-configuration", "net/antenna-configuration", client.Net.AntennaConfiguration)
	get("developer-mode", "developermode/developer-mode", client.Developer.DeveloperMode)
	get("developer-item", "developermode/developer-item", client.Developer.DeveloperItem)
	get("atport-status", "app/atport-status", client.Developer.AtportStatus)
	get("wlan-debug", "wlan/wlan-debug", client.WLan.WlanDebug)

	// ---- 写端点（默认不执行） ----
	anyWrite := setLockCell != "" || setAtport != "" || setWlanDebug != ""
	if anyWrite {
		fmt.Println("── 写端点 ──────────────────────────────")
	}
	if setLockCell != "" {
		var freq, pci int
		if _, err := fmt.Sscanf(setLockCell, "%d:%d", &freq, &pci); err != nil {
			fatal("set-lock-cell 格式应为 FREQ:PCI，收到 %q", setLockCell)
		}
		lock := 1
		if freq == 0 && pci == 0 {
			lock = 0
		}
		resp, err := client.Ntwk.LockCell(lock, freq, pci)
		report("lock-cell", fmt.Sprintf("net/lock-cell → %d:%d (%d)", freq, pci, lock), resp, err)
	}
	if setAtport != "" {
		var enable int
		switch setAtport {
		case "1", "0":
			fmt.Sscanf(setAtport, "%d", &enable)
		default:
			fatal("set-atport 应为 1 或 0，收到 %q", setAtport)
		}
		resp, err := client.Developer.SetAtportStatus(enable)
		report("atport-status", "app/atport-status (set)", resp, err)
	}
	if setWlanDebug != "" {
		fields := map[string]interface{}{}
		for _, kv := range strings.Split(setWlanDebug, ",") {
			parts := strings.SplitN(kv, "=", 2)
			if len(parts) != 2 {
				fatal("set-wlan-debug 应为 k=v,k2=v2，收到 %q", kv)
			}
			fields[parts[0]] = parts[1]
		}
		resp, err := client.WLan.SetWlanDebug(fields)
		report("wlan-debug", "wlan/wlan-debug (set)", resp, err)
	}
}

func report(name, endpoint string, data interface{}, err error) {
	if err != nil {
		fmt.Printf("  %-22s ✗ %v\n", endpoint, err)
		return
	}
	fmt.Printf("  %-22s ✅ %v\n", endpoint, data)
}

func fatal(format string, args ...interface{}) {
	fmt.Fprintf(os.Stderr, "✗ "+format+"\n", args...)
	os.Exit(1)
}
