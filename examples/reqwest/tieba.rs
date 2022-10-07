use reqwest;
use scraper::{Html, Selector};
use anyhow::{anyhow, Result};
use futures::stream::StreamExt;
use std::{borrow::Cow, path::{Path, PathBuf}};
use structopt::StructOpt;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};
use std::fs;

const TIEBA_URL: &str = "https://tieba.baidu.com/p/6105274031";


// async fn getMaxPage(url:&str) -> Result<&str,reqwest::Error>{
//     let resp = reqwest::get(TIEBA_URL).await?;
//     //println!("Body:{:#?}",resp.text().await?);
//     let body = resp.text().await?;
//     // println!("body:{}",&body);
//     let doc = Html::parse_fragment(&body);
//     let selector = Selector::parse(".jump_input_bright").unwrap();
//     for el in doc.select(&selector) {
//         // println!("img:{}",el.value().attr("max-page").unwrap());
//         return Ok(el.value().attr("max-page").unwrap())
//     }
//     Ok("1")
// }
#[tokio::main]
async fn main()  -> Result<(),reqwest::Error> {
    let mut paths: Vec<String> = Vec::new();
    // HTML
    let resp = reqwest::get(TIEBA_URL).await?;
    //println!("Body:{:#?}",resp.text().await?);
    let body = resp.text().await?;
    // println!("body:{}",&body);
    let doc = Html::parse_fragment(&body);
    let selector = Selector::parse(".BDE_Image").unwrap();
    for el in doc.select(&selector) {
        // println!("{}",el.value().attr("src").unwrap());
        paths.push(el.value().attr("src").unwrap().to_string());
    }
    let length = paths.len();
    let fetches = futures::stream::iter(paths.into_iter().enumerate().map(|(index, url)| {
        async move {
            let mut response  = get(&url).await?;
            let filename = basename(&url, '/');
            println!("[{:6}/{}] {}", index, length, filename);
            save(filename.as_ref(), &mut response).await?;
            Ok::<(), Box<dyn std::error::Error>>(())
        }
    }))
    .buffered(10)
    .collect::<Vec<_>>();
    fetches.await;

    Ok(())
}

fn basename(path: &str, sep: char) -> Cow<str> {
    let mut pieces = path.rsplit(sep);
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}

async fn get(url: &str) -> Result<reqwest::Response> {
    reqwest::get(url)
        .await
        .map_err(|e| anyhow!("Request url {} error: {}", url, e))
}

async fn save(filename: &str, response: &mut reqwest::Response) -> Result<()> {
    let mut options = OpenOptions::new();
    let mut file = options
        .append(true)
        .create(true)
        .read(true)
        .open(filename)
        .await?;

    while let Some(chunk) = &response.chunk().await.expect("Failed") {
        match file.write_all(&chunk).await {
            Ok(_) => {}
            Err(e) => return Err(anyhow!("File {} save error: {}", filename, e)),
        }
    }
    Ok(())
}