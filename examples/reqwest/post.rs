use std::collections::HashMap;
use reqwest::header::HeaderMap;
use serde_json::value::Value;


async fn get() -> Result<HashMap<String, String>, reqwest::Error>{
    Ok(reqwest::get("https://httpbin.org/ip").await?.json::<HashMap<String, String>>().await?)
}

async fn post() -> Result<HashMap<String, Value>, reqwest::Error>{
    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // 组装要提交的数据
    let mut data = HashMap::new();
    data.insert("user", "zhangsan");
    data.insert("password", "https://docs.rs/serde_json/1.0.59/serde_json/");

    // 发起post请求并返回
    Ok(client.post("https://httpbin.org/post").headers(headers).json(&data).send().await?.json::<HashMap<String, Value>>().await?)
}

#[tokio::main]
async fn main() {
    if let Ok(resp) = get().await {
        println!("{:#?}", resp);
    }

    if let Ok(res) = post().await {
        println!("{:#?}", res);
    }
}
