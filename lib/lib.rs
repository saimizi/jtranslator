pub mod error;
pub mod key_info;

use error::JTranslateError;
use error_stack::{Report, Result};
use jlogger_tracing::jdebug;
use key_info::key_info;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct TranslationEntry {
    text: String,
    to: String,
}

impl TranslationEntry {
    pub fn new(text: &str, to: &str) -> Self {
        TranslationEntry {
            text: text.to_owned(),
            to: to.to_owned(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn language(&self) -> &str {
        &self.to
    }
}

#[derive(Serialize, Deserialize)]
struct Translation {
    translations: Vec<TranslationEntry>,
}

pub async fn translate_text(
    text: &str,
    from: &str,
    to: Vec<&str>,
) -> Result<Vec<TranslationEntry>, JTranslateError> {
    let key_info = key_info()?;
    let url = format!("{}/translate?api-version=3.0", key_info.endpoint());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Ocp-Apim-Subscription-Key", key_info.key().parse().unwrap());
    headers.insert(
        "Ocp-Apim-Subscription-Region",
        key_info.region().parse().unwrap(),
    );
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let body = json!([{
        "text": text,
    }]);

    jdebug!("{body}");
    let mut query = vec![];
    query.push(("from", from));
    for t in to.iter() {
        query.push(("to", t));
    }

    let client = reqwest::Client::new();
    let request = client.post(&url).headers(headers).query(&query).json(&body);

    jdebug!("{:?}", request);
    let response = request
        .send()
        .await
        .map_err(|e| Report::new(JTranslateError::IOError).attach_printable(e))?;

    let translated_text = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| Report::new(JTranslateError::IOError).attach_printable(e))?
        .to_string();

    jdebug!("{}", translated_text);

    let value = json::parse(&translated_text)
        .map_err(|e| Report::new(JTranslateError::InvalidData).attach_printable(e))?;

    match value {
        json::JsonValue::Array(mut array) => {
            if let json::JsonValue::Object(o) = array
                .pop()
                .ok_or(Report::new(JTranslateError::InvalidData))?
            {
                if let Some(json::JsonValue::Array(translations)) = o.get("translations") {
                    let mut result = vec![];

                    for entry in translations.iter() {
                        if let json::JsonValue::Object(translation) = entry {
                            if let (Some(text), Some(language)) =
                                (translation.get("text"), translation.get("to"))
                            {
                                result.push(TranslationEntry::new(
                                    text.as_str().unwrap(),
                                    language.as_str().unwrap(),
                                ));
                            }
                        }
                    }

                    if !result.is_empty() {
                        return Ok(result);
                    }
                }
            }
        }
        json::JsonValue::Object(o) => {
            if let Some(json::JsonValue::Object(e)) = o.get("error") {
                if let (
                    Some(json::JsonValue::Number(code)),
                    Some(json::JsonValue::String(message)),
                ) = (e.get("code"), e.get("message"))
                {
                    return Err(Report::new(JTranslateError::InvalidData)
                        .attach_printable(format!("code: {code}, message: {message}")));
                }
            }
        }

        _ => {}
    }

    Err(Report::new(JTranslateError::InvalidData))
}
