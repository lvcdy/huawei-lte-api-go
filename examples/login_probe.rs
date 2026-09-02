//! Diagnostic: attempt login and dump the raw device error response.
//! Confirms the login request format is correct (even when the password is
//! wrong, the device should respond with a typed error code).

use huawei_lte_api::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://192.168.8.1/";
    let conn = Connection::new(base_url, None, None)?;

    let client = conn.session().client().clone();

    // Fetch CSRF tokens first (as the session does).
    let home = client.get(base_url).send()?;
    let text = home.text()?;
    let re = regex::Regex::new(r#"name="csrf_token"\s+content="(\S+)""#)?;
    let tokens: Vec<String> = re
        .captures_iter(&text)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .collect();
    println!("CSRF tokens from homepage: {tokens:?}");
    let token = tokens.last().cloned().unwrap_or_default();

    // Mirror what login sends: username/password_type + SHA256-derived password.
    // Password comes from the environment so this probe works with the real
    // device password (live_test does the same).
    let username = std::env::var("HUAWEI_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let password = std::env::var("HUAWEI_PASSWORD")?;
    let pwd_b64 = {
        use base64::Engine;
        use sha2::{Digest, Sha256};
        let hex1 = hex::encode(Sha256::digest(password.as_bytes()));
        let b64_hex = base64::engine::general_purpose::STANDARD.encode(hex1.as_bytes());
        let mut conc = Vec::new();
        conc.extend_from_slice(username.as_bytes());
        conc.extend_from_slice(b64_hex.as_bytes());
        conc.extend_from_slice(token.as_bytes());
        let hex2 = hex::encode(Sha256::digest(&conc));
        base64::engine::general_purpose::STANDARD.encode(hex2.as_bytes())
    };
    let body = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><request><Username>{username}</Username><Password>{pwd_b64}</Password><password_type>4</password_type></request>"
    );

    println!("\nPOST api/user/login (password_type=4 / SHA256)...");
    let resp = client
        .post(format!("{base_url}api/user/login"))
        .header("__RequestVerificationToken", &token)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()?;
    println!("HTTP status: {}", resp.status());
    println!("headers: {:?}", resp.headers());
    let bytes = resp.bytes()?.to_vec();
    println!("body utf8: {}", String::from_utf8_lossy(&bytes));
    Ok(())
}
