use reqwest::header::HeaderMap;

use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Response {
    result: isize,
    content: String,
}

async fn get(text: &str) -> Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36".parse().unwrap());
    let http_response = client
        .get(format!(
            "http://api.qingyunke.com/api.php?key=free&appid=0&msg={}",
            text
        ))
        .headers(headers)
        .send()
        .await?;
    let response = http_response.json::<Response>().await?;
    // Ok(client.get("http://api.qingyunke.com/api.php?key=free&appid=0&msg=%E4%BD%A0%E5%A5%BD").headers(headers).send().await?.text().await?)
    Ok(response)
}

#[tokio::main]
async fn main() {
    if let Ok(resp) = get("你好").await {
        println!("{:#?}", resp);
    }
}
