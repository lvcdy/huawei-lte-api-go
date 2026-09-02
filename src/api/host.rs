//! Host API group (`api/Host.py`).

use crate::connection::Connection;
use crate::errors::Result;
use crate::tools::map_of;
use crate::xml::XmlMap;

use super::post_set;

/// Host API group.
pub struct Host<'a> {
    conn: &'a Connection,
}

impl<'a> Host<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Host { conn }
    }

    /// Send host information to the server.
    ///
    /// The Python method takes a `datetime` and formats `Time` and `Timezone`
    /// from it; here the caller passes those two values pre-formatted.
    ///
    /// * `time` — current time as `%Y%m%d%H%M%S`.
    /// * `timezone` — `GMT` offset string (e.g. `GMT+0200`).
    /// * `platform` — platform information.
    /// * `user_agent` / `version` — client identification strings.
    pub fn info(
        &self,
        time: &str,
        timezone: &str,
        platform: &str,
        user_agent: &str,
        version: &str,
    ) -> Result<String> {
        let body: XmlMap = map_of([
            ("Time", time.to_string()),
            ("Timezone", timezone.to_string()),
            ("Platform", platform.to_string()),
            ("PlatformVer", user_agent.to_string()),
            ("Navigator", version.to_string()),
            ("NavigatorVer", user_agent.to_string()),
        ]);
        post_set(self.conn, "api/host/info", &body)
    }
}
