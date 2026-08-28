// cpedebug 以原始 HTTP 逐步复现 CPE 登录流程，打印每一步的请求与响应，
// 用于排查 H168-383 等设备的固件适配问题。
package main

import (
	"crypto/sha256"
	"encoding/base64"
	"encoding/hex"
	"flag"
	"fmt"
	"io"
	"net/http"
	"net/http/cookiejar"
	"net/url"
	"regexp"
	"strings"
	"time"
)

var csrfRe = regexp.MustCompile(`name="csrf_token"\s+content="(\S+)"`)

func main() {
	var rawURL string
	var username string
	var password string
	flag.StringVar(&rawURL, "url", "http://192.168.8.1/", "CPE 设备地址")
	flag.StringVar(&username, "username", "admin", "登录用户名")
	flag.StringVar(&password, "password", "", "登录密码")
	flag.Parse()

	// 使用 CookieJar —— 华为新固件把 CSRF token 绑定在 Session Cookie 上
	jar, _ := cookiejar.New(nil)
	client := &http.Client{Timeout: 10 * time.Second, Jar: jar}

	fmt.Println("=== Step 1: GET / (with CookieJar) ===")
	if !strings.HasSuffix(rawURL, "/") {
		rawURL += "/"
	}
	home := getRaw(client, rawURL, "")
	tokens := csrfRe.FindAllStringSubmatch(home, -1)
	fmt.Printf("csrf tokens from index: %d\n", len(tokens))
	printTokens(tokens)
	fmt.Printf("cookies: %v\n", jar.Cookies(mustURL(rawURL)))

	// Step 2: GET user/state-login（Python: len(tokens)==1 才带头，2 个 token 时不带！）
	fmt.Println("\n=== Step 2: GET user/state-login (NO token, mirror Python) ===")
	stateResp := getRawWithToken(client, rawURL, "api/user/state-login", "")
	fmt.Println(stateResp)

	// 尝试提取 state-login 里的 password_type / rsapadingtype
	passType := "0"
	rsaPadding := "0"
	state := ""
	for _, key := range []string{"password_type", "rsapadingtype", "State"} {
		if m := regexp.MustCompile("<" + key + ">([^<]*)</" + key + ">").FindStringSubmatch(stateResp); len(m) == 2 {
			fmt.Printf("  → %s = %q\n", key, m[1])
			switch key {
			case "password_type":
				passType = m[1]
			case "rsapadingtype":
				rsaPadding = m[1]
			case "State":
				state = m[1]
			}
		}
	}

	// Step 3: POST user/login（用 state-login 声明的 password_type 编码密码）
	fmt.Printf("\n=== Step 3: POST user/login (password_type=%s, rsa_padding=%s) ===\n", passType, rsaPadding)
	_ = state

	var encPass string
	if passType == "4" {
		// SHA256: base64(sha256(hex(sha256(password)) + token + username 组合后 sha256))
		encPass = encodePwdSHA256(username, password, firstToken(tokens))
		fmt.Printf("  编码后的密码（SHA256）: %s\n", encPass)
	} else {
		encPass = base64.StdEncoding.EncodeToString([]byte(password))
		fmt.Printf("  编码后的密码（Base64）: %s\n", encPass)
	}

	loginXML := fmt.Sprintf("<?xml version=\"1.0\" encoding=\"UTF-8\"?><request><Username>%s</Username><Password>%s</Password><password_type>%s</password_type></request>",
		username, encPass, passType)
	postRaw(client, rawURL, "api/user/login", loginXML, firstToken(tokens))

	fmt.Println("\n=== 说明 ===")
	fmt.Println("本工具通过真实 HTTP 请求逐步诊断 CPE 连接问题：")
	fmt.Println("  • CookieJar：华为新固件把 CSRF token 绑定在 SessionID Cookie 上，必须带 Cookie 才能通过 125003 校验")
	fmt.Println("  • GET state-login 不应带头（token 数>1 时不消费），POST login 用 token[0]")
	fmt.Println("  • 响应为 <response>OK</response> 即登录成功")
}

// mustURL 解析 URL，失败则 panic（用于 cookiejar 查询）。
func mustURL(s string) *url.URL {
	u, err := url.Parse(strings.TrimSuffix(s, "/"))
	if err != nil {
		panic(err)
	}
	return u
}

// encodePwdSHA256 实现 User._encode_password 的 SHA256 分支。
func encodePwdSHA256(username, password, csrfToken string) string {
	pwHash := sha256.Sum256([]byte(password))
	pwHex := hex.EncodeToString(pwHash[:])
	b64OfHex := base64.StdEncoding.EncodeToString([]byte(pwHex))

	concentrated := username + b64OfHex + csrfToken
	concHash := sha256.Sum256([]byte(concentrated))
	return base64.StdEncoding.EncodeToString([]byte(hex.EncodeToString(concHash[:])))
}

func printTokens(tokens [][]string) {
	for i, m := range tokens {
		fmt.Printf("  token[%d] = %s\n", i, m[1])
	}
}

func firstToken(tokens [][]string) string {
	if len(tokens) > 0 {
		return tokens[0][1]
	}
	return ""
}

func getRaw(client *http.Client, rawURL string, token string) string {
	req, _ := http.NewRequest("GET", rawURL, nil)
	if token != "" {
		req.Header.Set("__RequestVerificationToken", token)
	}
	resp, err := client.Do(req)
	if err != nil {
		fmt.Printf("  ✗ GET: %v\n", err)
		return ""
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	return string(body)
}

func getRawWithToken(client *http.Client, base, endpoint, token string) string {
	req, _ := http.NewRequest("GET", base+endpoint, nil)
	if token != "" {
		req.Header.Set("__RequestVerificationToken", token)
	}
	resp, err := client.Do(req)
	if err != nil {
		fmt.Printf("  ✗ GET %s: %v\n", endpoint, err)
		return ""
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	fmt.Printf("  GET %s → status=%d\n", endpoint, resp.StatusCode)
	return string(body)
}

func postRaw(client *http.Client, base, endpoint, body string, token string) {
	req, _ := http.NewRequest("POST", base+endpoint, strings.NewReader(body))
	req.Header.Set("Content-Type", "application/xml")
	if token != "" {
		req.Header.Set("__RequestVerificationToken", token)
	}
	resp, err := client.Do(req)
	if err != nil {
		fmt.Printf("  ✗ POST %s: %v\n", endpoint, err)
		return
	}
	defer resp.Body.Close()
	respBody, _ := io.ReadAll(resp.Body)
	fmt.Printf("  POST %s → status=%d ct=%s\n", endpoint, resp.StatusCode, resp.Header.Get("Content-Type"))
	fmt.Printf("  响应: %s\n", string(respBody))
	for _, h := range []string{"__RequestVerificationToken", "__RequestVerificationTokenone", "__RequestVerificationTokentwo"} {
		if v := resp.Header.Get(h); v != "" {
			fmt.Printf("  header %s = %s\n", h, v)
		}
	}
}
