//! Per-device traffic statistics for a real Huawei CPE.
//!
//! Logs in and calls `system/hostinfo` (`Client::system().hostinfo()`), which
//! returns a JSON array — one entry per host — including cumulative per-device
//! traffic (`TxKBytes` / `RxKBytes`) and current rates (`UpRate` / `DownRate`).
//! Prints a readable table sorted by total traffic.
//!
//! Usage:
//!   $env:HUAWEI_PASSWORD="..." ; cargo run --example device_traffic

// The header println! passes column titles as literals; clippy's
// print_literal lint wants to inline them, but they are headers.
#![allow(clippy::print_literal)]

use huawei_lte_api::{Client, Connection};

/// One host row: (name, mac, ip, iface, tx_kb, rx_kb, up_bps, down_bps, active, raw_json).
type HostRow = (
    String,
    String,
    String,
    String,
    u64,
    u64,
    u64,
    u64,
    bool,
    String,
);

/// Format a byte count (from KB) into a human-readable string.
fn fmt_kb(kb: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = kb as f64 * 1024.0;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.2} MB", b / MB)
    } else if b >= KB {
        format!("{:.1} KB", b / KB)
    } else {
        format!("{b} B")
    }
}

/// Format a rate (bytes/sec) into a human-readable string.
fn fmt_bps(bps: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    if bps >= MB {
        format!("{:.2} MB/s", bps as f64 / MB as f64)
    } else if bps >= KB {
        format!("{:.1} KB/s", bps as f64 / KB as f64)
    } else {
        format!("{bps} B/s")
    }
}

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

    println!("\n== Per-device traffic (system/hostinfo) ==");
    let hosts = client.system().hostinfo()?;

    // The endpoint returns a JSON array at the top level.
    let arr = match &hosts {
        serde_json::Value::Array(a) => a.clone(),
        other => vec![other.clone()],
    };

    // Gather rows, sorting by total traffic descending.
    let mut rows: Vec<HostRow> = Vec::new();
    for h in &arr {
        let mac = h
            .get("MACAddress")
            .and_then(|v| v.as_str())
            .unwrap_or("?")
            .to_string();
        let name = h
            .get("ActualName")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .or_else(|| h.get("HostName").and_then(|v| v.as_str()))
            .unwrap_or("?")
            .to_string();
        let ip = h
            .get("IPAddress")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .split(';')
            .next()
            .unwrap_or("")
            .to_string();
        let iface = h
            .get("InterfaceType")
            .and_then(|v| v.as_str())
            .unwrap_or("?")
            .to_string();
        let tx = h
            .get("TxKBytes")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let rx = h
            .get("RxKBytes")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let up = h.get("UpRate").and_then(|v| v.as_u64()).unwrap_or(0);
        let down = h.get("DownRate").and_then(|v| v.as_u64()).unwrap_or(0);
        let active = h.get("Active").and_then(|v| v.as_bool()).unwrap_or(false);
        rows.push((
            name,
            mac,
            ip,
            iface,
            tx,
            rx,
            up,
            down,
            active,
            h.to_string(),
        ));
    }
    // Sort by total (tx + rx) descending.
    rows.sort_by_key(|r| std::cmp::Reverse(r.4 + r.5));

    println!(
        "{:<28} {:<18} {:<16} {:<10} {:<12} {:<12} {:<10} {:<10}  {}",
        "设备", "MAC", "IP", "接口", "上行(Tx)", "下行(Rx)", "上行速率", "下行速率", "在线"
    );
    let divider = "-".repeat(150);
    println!("{divider}");
    for (name, mac, ip, iface, tx, rx, up, down, active, _raw) in &rows {
        println!(
            "{:<28} {:<18} {:<16} {:<10} {:<12} {:<12} {:<10} {:<10}  {}",
            name,
            mac,
            ip,
            iface,
            fmt_kb(*tx),
            fmt_kb(*rx),
            fmt_bps(*up),
            fmt_bps(*down),
            if *active { "●" } else { "○" }
        );
    }

    println!("\n== Done ==");
    Ok(())
}
