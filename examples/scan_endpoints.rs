//! Probe a wider set of candidate endpoints on the real CPE, to discover
//! which ones this firmware supports beyond the library's known set.
//!
//! Logs in, then raw-GETs a broad list of candidate paths and prints any that
//! return real data (non-error), plus a compact summary of error codes for
//! the rest. This helps find endpoints worth adding to the library.
//!
//! Usage:
//!   $env:HUAWEI_PASSWORD="..." ; cargo run --example scan_endpoints

use huawei_lte_api::{Client, Connection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://192.168.8.1/";
    println!("== Connecting to {base_url} ==");
    let conn = Connection::new(base_url, None, None)?;
    let client = Client::new(&conn);

    let username = std::env::var("HUAWEI_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let password = std::env::var("HUAWEI_PASSWORD")?;
    match client.user().login(&username, Some(&password), false) {
        Ok(true) => println!("[login] OK"),
        Ok(false) => println!("[login] already logged in"),
        Err(e) => {
            println!("[login] FAILED => {e}");
            return Ok(());
        }
    }

    let session = conn.session();
    // Clone the session's client so login cookies are shared; add a per-request
    // timeout because some endpoints hang (device waits forever for a POST body).
    let http = session.client().clone();
    let token = session.csrf_tokens().last().cloned();
    let timeout = std::time::Duration::from_secs(6);

    // Broad candidate list: known groups + likely firmware-specific paths.
    let candidates = [
        // --- monitoring ---
        "api/monitoring/status",
        "api/monitoring/traffic-statistics",
        "api/monitoring/month_statistics",
        "api/monitoring/month_statistics_wlan",
        "api/monitoring/start_date",
        "api/monitoring/start_date_wlan",
        "api/monitoring/check-notifications",
        "api/monitoring/converged-status",
        "api/monitoring/daily-data-limit",
        "api/monitoring/onekey_diag",
        "api/monitoring/statistic-feature-switch",
        "api/monitoring/wifi-month-setting",
        // --- device ---
        "api/device/information",
        "api/device/basic_information",
        "api/device/signal",
        "api/device/boot_time",
        "api/device/antenna_status",
        "api/device/antenna_settings",
        "api/device/antenna_type",
        "api/device/antenna_set_type",
        // note: device/control, device/mode, device/pwrmngment are write-only
        // (device hangs on GET), so skipped here.
        // --- wlan ---
        "api/wlan/basic-settings",
        "api/wlan/security-settings",
        "api/wlan/multi-basic-settings",
        "api/wlan/multi-security-settings",
        "api/wlan/multi-security-settings-ex",
        "api/wlan/multi-switch-settings",
        "api/wlan/multi-macfilter-settings",
        "api/wlan/multi-macfilter-settings-ex",
        "api/wlan/station-information",
        "api/wlan/host-list",
        "api/wlan/wifi-feature-switch",
        "api/wlan/handover-setting",
        "api/wlan/radio-settings",
        "api/wlan/wlan-debug",
        // --- net ---
        "api/net/current-plmn",
        "api/net/net-mode",
        "api/net/network",
        "api/net/net-mode-list",
        "api/net/register",
        "api/net/operator-list",
        "api/net/ps-switch",
        "api/net/antenna-configuration",
        "api/net/current-network",
        "api/net/cell-info",
        // note: net/lock-cell is a POST-only endpoint, skipped on GET.
        // --- ntwk ---
        "api/ntwk/dialup-connection",
        "api/ntwk/dialup-connection-settings",
        "api/ntwk/dialup-connection-state",
        "api/ntwk/host-list",
        "api/ntwk/net-mode",
        "api/ntwk/network",
        // --- lan ---
        "api/lan/HostInfo",
        "api/lan/hosts-info",
        "api/lan/host-list",
        "api/lan/device-info",
        // --- dhcp ---
        "api/dhcp/dhcp-host-info",
        "api/dhcp/settings",
        "api/dhcp/dhcp-all-ip",
        // --- system ---
        "api/system/HostInfo",
        "api/system/deviceinfo",
        "api/system/deviceinfoex",
        "api/system/devcapacity",
        "api/system/onlinestate",
        // --- statistic ---
        "api/statistic/feature-roam-statistic",
        "api/statistic/host-statistics",
        "api/statistic/host-traffic",
        "api/statistic/traffic-statistics",
        // --- security ---
        "api/security/security",
        "api/security/firewall",
        "api/security/mac-filter",
        "api/security/upnp",
        // --- usermanual ---
        "api/usermanual/public-sys-resources",
        // --- sms ---
        "api/sms/sms-count",
        "api/sms/sms-list",
        "api/sms/sms-status",
        // --- cwmp ---
        "api/cwmp/tr069",
        "api/cwmp/wan-config",
        // --- global ---
        "api/global/network-mode",
        // --- ota / update ---
        "api/ota/status",
        "api/online-update/status",
        // --- pin ---
        "api/pin/pin-status",
        "api/pin/operate",
        // --- led ---
        "api/led/led-status",
        "api/led/led-switch",
        // --- diagnosis ---
        "api/diagnosis/status",
        "api/diagnosis/traceroute-status",
        // --- ussd ---
        "api/ussd/status",
        // --- time ---
        "api/time/time",
        "api/time/current-time",
        // --- voice ---
        "api/voice/voice",
        // --- log ---
        "api/log/log",
        // --- pb ---
        "api/pb/pb",
        // --- developer ---
        "api/developer/developer-mode",
        "api/developermode/developer-mode",
        "api/developer/atport-status",
        // --- cradle ---
        "api/cradle/status",
        // --- sd-card ---
        "api/sd-card/sd-card",
        // --- vpn ---
        "api/vpn/vpn",
        "api/vpn/settings",
        // --- web-server ---
        "api/web-server/status",
        // --- app ---
        "api/app/status",
        "api/app/atport-status",
    ];

    let mut data_ok = 0;
    let mut not_found = 0;
    let mut forbidden = 0;
    for path in candidates {
        let url = format!("{base_url}{path}");
        let mut req = http.get(&url).timeout(timeout);
        if let Some(t) = &token {
            req = req.header("__RequestVerificationToken", t);
        }
        let resp = match req.send() {
            Ok(r) => r,
            Err(e) => {
                println!("[{}] TIMEOUT/ERR: {e}", path);
                continue;
            }
        };
        let ct = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .map(|v| v.to_str().unwrap_or("?").to_string())
            .unwrap_or_else(|| "(none)".to_string());
        let bytes = match resp.bytes() {
            Ok(b) => b.to_vec(),
            Err(e) => {
                println!("[{}] READ ERR: {e}", path);
                continue;
            }
        };
        let body = String::from_utf8_lossy(&bytes).to_string();
        let is_error = body.contains("<error>") && body.contains("<code>");
        let code = if is_error {
            body.split("<code>")
                .nth(1)
                .and_then(|s| s.split("</code>").next())
                .unwrap_or("?")
                .to_string()
        } else {
            String::new()
        };
        if is_error {
            match code.as_str() {
                "100003" => forbidden += 1,
                _ => not_found += 1,
            }
            if code != "100003" {
                println!("[{}] ERR code={code} ({} bytes)", path, bytes.len());
            }
        } else {
            data_ok += 1;
            println!("\n[{}] DATA ({}) {}", path, ct, bytes.len());
            println!("  {}", body.chars().take(220).collect::<String>());
        }
    }
    println!(
        "\n== Summary: {} DATA endpoints, {} forbidden(100003), {} other errors ==",
        data_ok, forbidden, not_found
    );
    Ok(())
}
