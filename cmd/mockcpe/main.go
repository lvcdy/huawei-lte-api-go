// mockcpe 一个极简的华为 CPE mock 服务器，用于端到端验证 cpetest 工具。
// 仅实现 cpetest 需要的端点；响应内容参考真实设备格式。
//
// 用法：go run ./cmd/mockcpe :8080
package main

import (
	"fmt"
	"net/http"
	"os"
)

func main() {
	addr := ":8080"
	if len(os.Args) > 1 {
		addr = os.Args[1]
	}
	mux := http.NewServeMux()

	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/html")
		fmt.Fprint(w, `<html><head><meta name="csrf_token" content="mock-token"></head></html>`)
	})

	mux.HandleFunc("/api/device/seccellinfo", xml(`<response><nrseccell_list>168,78,0,501,440,-80,-95,-11,8</nrseccell_list><cell_id>0</cell_id></response>`))
	mux.HandleFunc("/api/device/nbrcellinfo", xml(`<response><nbrcell_nrlist>168,78,0,502,441,-75,-90,-10,9;169,79,1,503,442,-70,-85,-9,10</nbrcell_nrlist></response>`))
	mux.HandleFunc("/api/net/antenna-configuration", xml(`<response><antenna_mode>0</antenna_mode><gain>3.5</gain><mode_number>4</mode_number></response>`))
	mux.HandleFunc("/api/developermode/developer-mode", xml(`<response><developer_mode>0</developer_mode></response>`))
	mux.HandleFunc("/api/developermode/developer-item", xml(`<response><telnet_enable>0</telnet_enable><at_enable>0</at_enable></response>`))
	mux.HandleFunc("/api/app/atport-status", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost {
			w.WriteHeader(http.StatusOK)
			fmt.Fprint(w, `<response>OK</response>`)
			return
		}
		xml(`<response><atport_status>0</atport_status></response>`)(w, r)
	})
	mux.HandleFunc("/api/wlan/wlan-debug", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost {
			fmt.Fprint(w, `<response>OK</response>`)
			return
		}
		xml(`<response><telnet_enable>0</telnet_enable><developermode_enable>0</developermode_enable></response>`)(w, r)
	})
	mux.HandleFunc("/api/net/lock-cell", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost {
			fmt.Fprint(w, `<response>OK</response>`)
			return
		}
		xml(`<response><lockcell>0</lockcell><freq>0</freq><pci>0</pci></response>`)(w, r)
	})

	fmt.Printf("mock CPE 服务器监听 %s\n", addr)
	if err := http.ListenAndServe(addr, mux); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func xml(body string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/xml")
		fmt.Fprint(w, body)
	}
}
