//! Live connectivity test against a real Huawei CPE.
//!
//! Read-only by default: exercises the session/CSRF/XML/JSON pipeline against
//! a real device without changing any settings.
//!
//! Usage:
//!   cargo run --example live_test
//!   # optional: provide password via env to also test authenticated endpoints
//!   $env:HUAWEI_PASSWORD="..." ; cargo run --example live_test

use huawei_lte_api::{Client, Connection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let base_url = "http://192.168.8.1/";
    println!("== Connecting to {base_url} ==");
    let connection = Connection::new(base_url, None, None)?;
    println!("[OK] Connection created");

    let client = Client::new(&connection);

    // ---- Public / unauthenticated read-only probes ----
    println!("\n== Unauthenticated probes ==");

    macro_rules! probe {
        ($name:expr, $expr:expr) => {{
            match $expr {
                Ok(v) => println!("[{}] OK => {}", $name, v),
                Err(e) => println!("[{}] ERR => {e}", $name),
            }
        }};
    }

    probe!(
        "device/basic_information",
        client.device().basic_information()
    );
    probe!("device/information", client.device().information());
    probe!("device/signal", client.device().signal());
    probe!("monitoring/status", client.monitoring().status());
    probe!(
        "monitoring/check-notifications",
        client.monitoring().check_notifications()
    );
    probe!("user/state-login", client.user().state_login());
    probe!("net/current-plmn", client.net().current_plmn());

    // ---- Optional authenticated probes (needs HUAWEI_PASSWORD env) ----
    if let Ok(password) = std::env::var("HUAWEI_PASSWORD") {
        let username = std::env::var("HUAWEI_USERNAME").unwrap_or_else(|_| "admin".to_string());
        println!("\n== Authenticated probes (password provided) ==");
        match client.user().login(&username, Some(&password), false) {
            Ok(true) => println!("[login] OK"),
            Ok(false) => println!("[login] already logged in"),
            Err(e) => {
                println!("[login] FAILED => {e}");
                println!("(密码可能不正确；请检查 HUAWEI_PASSWORD)");
                return Ok(());
            }
        }

        probe!("device/information (authed)", client.device().information());
        probe!("device/boot_time", client.device().boot_time());
        probe!("monitoring/status (authed)", client.monitoring().status());
        probe!(
            "monitoring/traffic-statistics",
            client.monitoring().traffic_statistics()
        );
        probe!("device/antenna_status", client.device().antenna_status());
        probe!(
            "device/get_antenna_settings",
            client.device().get_antenna_settings()
        );
        probe!("device/antenna_type", client.device().antenna_type());
        probe!(
            "device/antenna_set_type",
            client.device().antenna_set_type()
        );
        probe!("net/net-mode", client.net().net_mode());
        probe!("net/network", client.net().network());
        probe!("wlan/basic_settings", client.wlan().basic_settings());
        probe!("sms/sms-count", client.sms().sms_count());

        println!("\n== Logout ==");
        let out = client.user().logout();
        println!("[logout] {:?}", out);
    } else {
        println!("\n(未提供 HUAWEI_PASSWORD 环境变量，跳过已认证端点测试)");
    }

    println!("\n== Done ==");
    Ok(())
}
