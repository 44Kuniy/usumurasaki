use serde_json::json;

const OPEN_AI_TOKEN: &'static str = "Bearer xxx";

#[derive(Debug, Clone)]
pub struct GptClient {
    client: reqwest::Client,
}

impl GptClient {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_DISPOSITION,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_static(OPEN_AI_TOKEN),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        Self { client }
    }

    pub async fn get(
        &self,
        content: impl Into<String>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .get("https://api.openai.com/v1/chat/completions")
            .body(GptClient::body(content.into()))
            .send()
            .await
    }

    pub async fn post(
        &self,
        content: impl Into<String>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .post("https://api.openai.com/v1/chat/completions")
            .body(GptClient::body(content.into()))
            .send()
            .await
    }

    fn body(content: String) -> String {
        json!({
          "model": "gpt-3.5-turbo",
          "messages": [{"role": "user", "content": content}],
          "temperature": 0.7
        })
        .to_string()
    }
}
