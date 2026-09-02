//! SMS related enums.

/// SMS storage/format flags, mirroring the Python `SmsFormat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmsFormat {
    /// `"0"` — text message.
    Text = 0,
    /// `"1"` — PDU (binary) message.
    Pdu = 1,
}

impl SmsFormat {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

/// SMS box to operate on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmsBox {
    /// `"1"` — the inbox.
    Inbox = 1,
    /// `"2"` — the outbox / sent.
    Sent = 2,
    /// `"3"` — drafts.
    Draft = 3,
    /// `"4"` — deleted items.
    Deleted = 4,
}

impl SmsBox {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}
