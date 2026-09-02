//! Diagnostic: dump raw response bytes + content-type for a few endpoints.
//! Helps diagnose the CESU-8 / GBK decoding of Chinese text in JSON responses.

use huawei_lte_api::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://192.168.8.1/";
    let conn = Connection::new(base_url, None, None)?;
    let client = conn.session().client().clone();

    for path in [
        "api/net/current-plmn",
        "api/device/basic_information",
        "api/monitoring/status",
    ] {
        let url = format!("{base_url}{path}");
        let resp = client.get(&url).send()?;
        let ct = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .map(|v| v.to_str().unwrap_or("?").to_string())
            .unwrap_or_else(|| "(none)".to_string());
        let bytes = resp.bytes()?.to_vec();
        println!("\n== {path} == content-type: {ct}  ({} bytes)", bytes.len());
        println!(
            "raw hex  : {}",
            bytes
                .iter()
                .map(|b| format!("{b:02x}"))
                .take(120)
                .collect::<Vec<_>>()
                .join(" ")
        );
        println!(
            "utf8 lossy: {}",
            String::from_utf8_lossy(&bytes)
                .chars()
                .take(160)
                .collect::<String>()
        );
    }
    Ok(())
}
