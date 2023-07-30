use serde_json::{json, Value};
use w_config::load_config;

#[derive(Debug, Clone)]
pub struct GptClient {
    client: reqwest::Client,
}

impl GptClient {
    pub fn new(token: String) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_DISPOSITION,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        Self { client }
    }

    pub async fn post(
        &self,
        content: impl Into<String>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .post("https://api.openai.com/v1/chat/completions")
            .json(&GptClient::body_json(content.into()))
            .send()
            .await
    }

    fn body_json(content: String) -> Value {
        json!({
          "model": "gpt-3.5-turbo",
          "messages": [{"role": "user", "content": content}],
          "temperature": 0.7
        })
    }
}

#[tokio::test]
async fn test_gpt_client_post() -> anyhow::Result<()> {
    let config = load_config().unwrap();
    let client = GptClient::new(config.gpt_api_key);
    let res = client.post("
        下記はショッピングサイトAmazonの製品のUrlです。
        このUrlに含まれる文字列に含まれる情報を、JSON形式で多く教えてください。
        Keyは ASIN, genre,category, product_name を含んでください。
        genreは複数あることが予想されるので配列として出力してください。
        https://www.amazon.co.jp/REMYS-%E3%83%9E%E3%83%95%E3%83%A9%E3%83%BC-%E3%83%AC%E3%83%87%E3%82%A3%E3%83%BC%E3%82%B9-%E5%A5%B3%E6%80%A7%E3%81%AE%E7%A7%8B%E3%81%A8%E5%86%AC%E3%81%AE%E3%82%A6%E3%83%BC%E3%83%AB%E3%82%B9%E3%82%AB%E3%83%BC%E3%83%95%E3%83%8B%E3%83%83%E3%83%88%E6%89%8B%E4%BD%9C%E3%82%8A%E3%81%AE%E3%82%BD%E3%83%AA%E3%83%83%E3%83%89%E3%82%AB%E3%83%A9%E3%83%BC%E3%82%B9%E3%82%AB%E3%83%BC%E3%83%95-Color/dp/B0C1BT846S/ref=sr_1_2?keywords=%E6%89%8B%E4%BD%9C%E3%82%8A%E3%83%9E%E3%83%95%E3%83%A9%E3%83%BC&qid=1690505381&sr=8-2
    ").await.unwrap();

    let json: Value = res.json().await.unwrap();
    let _message = json["choices"][0]["message"]["content"].as_str().unwrap();

    Ok(())
}
