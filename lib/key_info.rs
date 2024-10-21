use crate::error::JTranslateError;
use error_stack::{Report, Result};
use jlogger_tracing::{jdebug, jerror};
use once_cell::sync::Lazy;
use std::path::Path;

static KEY_INFO: Lazy<KeyInfo> = Lazy::new(|| {
    let mut key_file = String::new();

    if let Ok(k) = std::env::var("JTRANSLATE_KEY_FILE") {
        if let Some(stripped) = k.strip_prefix("~/") {
            if let Ok(home) = std::env::var("HOME") {
                key_file = format!("{}/{}", home, stripped);
            }
        } else {
            key_file = k;
        }
    } else {
        if let Ok(home) = std::env::var("HOME") {
            key_file = format!("{}/.jtranslator", home);
        }
    }

    jdebug!(key_file = key_file);
    let region = "japaneast";
    let endpoint = "https://api.cognitive.microsofttranslator.com/";

    let key_file_path = Path::new(&key_file);
    if key_file_path.is_file() || key_file_path.is_symlink() {
        if let Ok(key) = std::fs::read_to_string(key_file_path) {
            return KeyInfo::new(&key, region, endpoint);
        }
    }

    jerror!("Can not access key file. Set JTRANSLATE_KEY_FILE correctly?");
    KeyInfo::new("INVALID", region, endpoint)
});

pub struct KeyInfo {
    key: String,
    region: String,
    endpoint: String,
}

impl KeyInfo {
    pub fn new(key: &str, region: &str, endpoint: &str) -> Self {
        Self {
            key: key.trim().to_owned(),
            region: region.trim().to_owned(),
            endpoint: endpoint.trim().to_owned(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn region(&self) -> &str {
        &self.region
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

pub fn key_info() -> Result<&'static KeyInfo, JTranslateError> {
    if KEY_INFO.key() == "INVALID" {
        Err(Report::new(JTranslateError::InvalidKey))
    } else {
        Ok(&KEY_INFO)
    }
}
