use std::str::FromStr;
use reqwest;

#[tokio::main]
pub async fn main() {
    let url = "http://mockbin.com/har";

    let client = reqwest::Client::new();
    let response = client.request(reqwest::Method::from_str("PROPFIND").unwrap(), url)
        .send()
        .await;

    let results = response.unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    dbg!(results);
}
