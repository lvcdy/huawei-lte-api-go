//! Diagnostic: probe the CPE for per-device traffic statistics endpoints.
//!
//! Logs in, then raw-GETs a list of candidate endpoints and dumps what the
//! device returns, so we can see which "per-device traffic" endpoints this
//! firmware actually supports.
//!
//! Usage:
//!   $env:HUAWEI_PASSWORD="..." ; cargo run --example probe_traffic

use huawei_lte_api::{Client, Connection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://192.168.8.1/";
    println!("== Connecting to {base_url} ==");
    let conn = Connection::new(base_url, None, None)?;
    let client = Client::new(&conn);

    let username = std::env::var("HUAWEI_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let password = std::env::var("HUAWEI_PASSWORD")?;
    println!("[login] {username} ...");
    match client.user().login(&username, Some(&password), false) {
        Ok(true) => println!("[login] OK"),
        Ok(false) => println!("[login] already logged in"),
        Err(e) => {
            println!("[login] FAILED => {e}");
            return Ok(());
        }
    }

    let session = conn.session();
    let http = session.client().clone();
    let token = session.csrf_tokens().last().cloned();

    // Candidate per-device / per-host traffic endpoints (best-effort probe).
    let candidates = [
        "api/wlan/multi-basic-settings",
        "api/wlan/station-information",
        "api/wlan/host-list",
        "api/lan/HostInfo",
        "api/dhcp/dhcp-host-info",
        "api/system/HostInfo",
        "api/monitoring/traffic-statistics",
        "api/monitoring/month_statistics",
        "api/monitoring/month_statistics_wlan",
        "api/device/host_info",
        "api/device/hostList",
        "api/device/multi-basic-settings",
        "api/statistic/host-statistics",
        "api/statistic/host_traffic",
        "api/host/device-statistics",
        "api/host/traffic-statistics",
        "api/lan/device-info",
        "api/lan/traffic-statistics",
        "api/dhcp/dhcp-host-traffic",
        "api/ntwk/host-list",
    ];

    for path in candidates {
        let url = format!("{base_url}{path}");
        let mut req = http.get(&url);
        if let Some(t) = &token {
            req = req.header("__RequestVerificationToken", t);
        }
        let resp = req.send()?;
        let ct = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .map(|v| v.to_str().unwrap_or("?").to_string())
            .unwrap_or_else(|| "(none)".to_string());
        let status = resp.status();
        let bytes = resp.bytes()?.to_vec();
        let body = String::from_utf8_lossy(&bytes).to_string();
        // Summarize: only show bodies that look like real data (not generic errors).
        let interesting = !(body.contains("<code>100003</code>")
            || body.contains("<code>100002</code>")
            || body.contains("<code>100001</code>")
            || body.contains("Error"));
        println!("\n== {path} ==");
        println!(
            "  HTTP {status}  content-type: {ct}  ({} bytes){}",
            bytes.len(),
            if interesting { "  <== DATA" } else { "" }
        );
        if interesting {
            println!("  body: {body}");
        }
    }

    println!("\n== Done ==");
    Ok(())
}
