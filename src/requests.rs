use crate::{constants, load_config, Actions};
use serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtractError {
    #[error("missing field")]
    MissingField,
    #[error("wrong type")]
    WrongType,
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json extraction failed: {0}")]
    Extract(#[from] ExtractError),
}

pub struct ModelAnswer {
    pub id: String,
    pub content: String,
}

fn extract_answer(v: &serde_json::Value) -> Result<ModelAnswer, ExtractError> {
    let id = v
        .get("id")
        .and_then(|x| x.as_str())
        .ok_or(ExtractError::MissingField)?;

    // OpenAI-style responses have choices as an array, so check index 0
    let content = v
        .get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content").and_then(|c| c.as_str()))
        .ok_or(ExtractError::MissingField)?;

    Ok(ModelAnswer {
        id: id.to_string(),
        content: content.to_string(),
    })
}

pub async fn request(text: &str, a: Actions) -> Result<ModelAnswer, RequestError> {
    let action: &str;
    let config = load_config();

    match a {
        Actions::Summarize => action = "Summarize the below text:",
        Actions::Explain => action = "Explain the following concept:",
    }

    let response: serde_json::Value = reqwest::Client::new()
        .post(constants::LLM_API_URL)
        .header("Authorization", format!("Bearer {}", config.llm_key))
        .json(&serde_json::json!({
          "messages": [
            { "role": "system", "content": constants::SYSTEM_PROMPT },
            { "role": "user", "content": format!("{}\n{}", action, text) }
          ],
          "model": constants::MODEL,
          "temperature": 1,
          "max_completion_tokens": 1024,
          "top_p": 1,
          "stream": false,
          "stop": null
        }))
        .send()
        .await?
        .json()
        .await?;

    extract_answer(&response).map_err(RequestError::from)
}
