use crate::{constants, Actions};
use serde_json;

#[derive(Debug)]
enum ExtractError {
    MissingField(()),
    WrongType(()),
}

// convert from serde_json::Error
impl From<serde_json::Error> for ExtractError {
    fn from(_: serde_json::Error) -> Self {
        ExtractError::WrongType(())
    }
}

pub struct ModelAnswer {
    pub id: String,
    pub content: String,
}

fn extract_answer<'a>(v: &serde_json::Value) -> Result<ModelAnswer, ExtractError> {
    let id = v
        .get("id")
        .and_then(|x| x.as_str())
        .ok_or(ExtractError::MissingField(()))?;

    let content = v
        .get("choices")
        .and_then(|c| {
            c.get("message")
                .and_then(|m| m.get("content").and_then(|c| c.as_str()))
        })
        .ok_or(ExtractError::MissingField(()))?;

    Ok(ModelAnswer {
        id: id.to_string(),
        content: content.to_string(),
    })
}

pub async fn request(text: &str, a: Actions) -> Option<ModelAnswer> {
    let action: &str;

    match a {
        Actions::Summarize => action = "Summarize the below text:",
        Actions::Explain => action = "Explain the following concept:",
    }

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
              "content": format!("{}\n{}",action, text)
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

    let model_answer = extract_answer(&response);

    match model_answer {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}
