use std::str::FromStr;

use crate::constants;
use serde_json;

#[derive(Debug)]
enum ExtractError {
    MissingField(&'static str),
    WrongType(&'static str),
}

// convert from serde_json::Error
impl From<serde_json::Error> for ExtractError {
    fn from(_: serde_json::Error) -> Self {
        ExtractError::WrongType("invalid JSON")
    }
}

struct ModelAnswer {
    id: String,
    content: String,
}

fn extract_answer(v: &serde_json::Value) -> Result<ModelAnswer, ExtractError> {
    let id = v
        .get("id")
        .and_then(|x| x.as_str())
        .ok_or(ExtractError::MissingField("id"))?;

    Ok(ModelAnswer {
        id: id.to_string(),
        content: String::from_str("dd").unwrap(),
    })
}

async fn summarize(text: &str) -> Option<ModelAnswer> {
    let response: serde_json::Value = reqwest::Client::new()
        .post(constants::LLM_API_URL)
        .json(&serde_json::json!({
          "messages": [
            {
                "role": "system",
                "content": constants::SYSTEM_PROMPT
            },
            {
              "role": "user",
              "content": text
            }
          ],
          "model": constants::MODEL,
          "temperature": 1,
          "max_completion_tokens": 1024,
          "top_p": 1,
          "stream": true,
          "stop": null
        }))
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    match extract_answer(&response) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}
