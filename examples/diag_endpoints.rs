//! Diagnostic: login with the library, then dump the RAW device XML response
//! for a set of endpoints that returned errors through the normal API layer.
//!
//! This distinguishes "device firmware returned an error" from "library
//! parsing bug": we bypass the API group wrappers and issue raw GETs with the
//! session's CSRF token, printing exactly what the device sends back.
//!
//! Usage:
//!   $env:HUAWEI_PASSWORD="..." ; cargo run --example diag_endpoints

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

    // Raw GET with the session's CSRF token (bypasses API wrappers).
    let session = conn.session();
    let http = session.client().clone();
    let token = session.csrf_tokens().last().cloned();

    for path in [
        "api/device/antenna_status",
        "api/device/antenna_settings",
        "api/device/antenna_type",
        "api/device/antenna_set_type",
        "api/wlan/basic_settings",
        "api/device/information",
    ] {
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
        println!("\n== {path} ==");
        println!(
            "  HTTP {status}  content-type: {ct}  ({} bytes)",
            bytes.len()
        );
        println!("  body: {}", String::from_utf8_lossy(&bytes));
    }

    println!("\n== Done ==");
    Ok(())
}
