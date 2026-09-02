//! Client-related enums.

/// HTTP request method used for API calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
        }
    }
}

/// The web UI language to request from the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// English
    En,
    /// Chinese
    Zh,
    /// Czech
    Cs,
    /// Russian
    Ru,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Zh => "zh",
            Language::Cs => "cs",
            Language::Ru => "ru",
        }
    }
}
