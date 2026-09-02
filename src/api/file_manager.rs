//! FileManager API group (`api/FileManager.py`).

use crate::connection::Connection;

/// FileManager API group.
///
/// Firmware upload is not yet supported: the Python `upload` method uses
/// `post_file` (multipart file upload), for which there is no Rust helper.
pub struct FileManager<'a> {
    #[allow(dead_code)]
    conn: &'a Connection,
}

impl<'a> FileManager<'a> {
    /// Build a group bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        FileManager { conn }
    }
}
