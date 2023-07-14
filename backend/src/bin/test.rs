use reqwest::Response;

const TWITTER_API_BASE_URL: &'static str = "https://test";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = TwitterRequestClient::get("waa").await?;
    println!("res ❓: {:#?}", res);
    Ok(())
}

pub struct TwitterRequestClient {}

impl TwitterRequestClient {
    async fn get(relative_path: &'static str) -> Result<Response, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder().build().unwrap();
        let req = client
            .get(format!("{}/{}", TWITTER_API_BASE_URL, relative_path))
            .send()
            .await?;

        Ok(req)
    }
}
