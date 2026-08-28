// cpecheck 连接华为 CPE 设备进行连通性测试。
//
// 用法：
//   go run ./cmd/cpecheck [URL] [--username=admin] [--password=xxxx]
//
// 缺省参数时使用 http://192.168.8.1/ 匿名访问（signal 等接口无需登录）。
package main

import (
	"flag"
	"fmt"
	"net/http"
	"os"
	"time"

	"github.com/lvcdy/huawei-lte-api-go"
	"github.com/lvcdy/huawei-lte-api-go/session"
)

func main() {
	var rawURL string
	var username string
	var password string
	flag.StringVar(&rawURL, "url", "http://192.168.8.1/", "CPE 设备地址")
	flag.StringVar(&username, "username", "", "登录用户名（可留空走匿名/URL 内嵌）")
	flag.StringVar(&password, "password", "", "登录密码")
	flag.Parse()

	// 与 Python 版示例一致的超时
	const timeout = 10 * time.Second

	var c interface {
		Close()
	}
	var client *huaweilteapi.Client

	if username != "" || password != "" {
		conn, err := session.NewConnection(rawURL, username, password, timeout, &http.Client{})
		if err != nil {
			fmt.Fprintf(os.Stderr, "连接失败: %v\n", err)
			os.Exit(1)
		}
		c = conn
		client = huaweilteapi.NewClient(conn)
		fmt.Println("✅ 已建立带认证连接（自动登录）")
	} else {
		s, err := session.NewSession(rawURL, timeout, &http.Client{})
		if err != nil {
			fmt.Fprintf(os.Stderr, "连接失败: %v\n", err)
			os.Exit(1)
		}
		c = s
		client = huaweilteapi.NewClient(&session.Connection{Session: s})
		fmt.Println("✅ 已建立匿名连接")
	}
	defer c.Close()

	// 1) 设备状态/信号（部分设备免登录，部分需要登录）
	status, statusErr := client.Monitoring.Status()
	reportEndpoint("monitoring/status", status, statusErr)

	signal, signalErr := client.Device.Signal()
	reportEndpoint("device/signal", signal, signalErr)

	info, infoErr := client.Device.Information()
	reportEndpoint("device/information", info, infoErr)

	// 只要有一个端点拿到真实数据即认为链路OK
	if statusErr == nil && signalErr == nil && infoErr == nil {
		fmt.Println("✅ 与 CPE 通信正常，各端点均有响应")
		os.Exit(0)
	} else if statusErr == nil || signalErr == nil || infoErr == nil {
		fmt.Println("⚠️ 部分端点响应，链路易通，未登录的端点返回权限错误")
		os.Exit(0)
	}
	fmt.Println("✗ 所有端点均失败，请检查设备地址/网络/凭据")
	os.Exit(1)
}

func reportEndpoint(name string, data map[string]interface{}, err error) {
	if err != nil {
		fmt.Printf("  %-24s ✗ %v\n", name, err)
		return
	}
	fmt.Printf("  %-24s ✅ %v\n", name, data)
}