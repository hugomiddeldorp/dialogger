use serde_json::Value;
use serde::{ Serialize, Deserialize };

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    steps: Vec<Step>,
}

#[derive(Deserialize, Debug)]
struct Step {
    content: Option<Vec<ContentBlock>>,
}

#[derive(Deserialize, Debug)]
struct ContentBlock {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dialogue {
    pub title: String,
    pub people: [String; 2],
    pub dialogue: Vec<String>,
}

fn extract_dialogue(resp: GeminiResponse) -> anyhow::Result<Dialogue> {
    let text = resp
        .steps
        .into_iter()
        .find_map(|step| {
          step.content
            .and_then(|c| c.into_iter().next())
            .map(|block| block.text)
        })
        .ok_or_else(|| anyhow::anyhow!("no dialogue content found"))?;

    let dialogue = serde_json::from_str(&text)?;
    Ok(dialogue)
}

pub async fn generate_dialogue(api_key: String, request_body: Value) -> anyhow::Result<Dialogue> {
  // TODO: this is very fragile if Gemini doesn't return exactly the correct format
  let client = reqwest::Client::new();
  let response: GeminiResponse = client.post("https://generativelanguage.googleapis.com/v1beta/interactions")
    .json(&request_body)
    .header("x-goog-api-key", api_key)
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

  extract_dialogue(response)
}
