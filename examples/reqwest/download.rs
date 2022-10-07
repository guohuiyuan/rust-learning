use anyhow::{anyhow, Result};
use futures::stream::StreamExt;
use std::{borrow::Cow, path::{Path, PathBuf}};
use structopt::StructOpt;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

fn basename(path: &str, sep: char) -> Cow<str> {
    let mut pieces = path.rsplit(sep);
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}
/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let inputfile = opt.input;
    let debug = opt.debug;
    println!("begin to download file {:?} ...", inputfile);
    let paths: Vec<String> = read_lines(inputfile).await?;
    let length = paths.len();
    let fetches = futures::stream::iter(paths.into_iter().enumerate().map(|(index, url)| {
        async move {
            let mut response  = get(&url).await?;
            let filename = basename(&url, '/');
            if debug {
                println!("[{:6}/{}] {}", index, length, filename);
            }
            save(filename.as_ref(), &mut response).await?;
            Ok::<(), Box<dyn std::error::Error>>(())
        }
    }))
    .buffered(10)
    .collect::<Vec<_>>();
    fetches.await;

    Ok(())
}

async fn read_lines<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<String>> {
    let file = OpenOptions::new().read(true).open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut res = Vec::new();
    while let Some(line) = lines.next_line().await? {
        res.push(line);
    }
    Ok(res)
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