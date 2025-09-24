use crate::constants;

async fn summarize(text: &str) -> &str {
    let response: serde_json::Value = reqwest::Client::new()
        .post(constants::LLM_API_URL)
        .json(&serde_json::json! {
          "messages": [
            {
                "role": "system",
                "content": constants::SYSTEM_PROMPT
            }
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
        })
        .send()
        .await?
        .json()
        .await?;

    return "ok";
}
