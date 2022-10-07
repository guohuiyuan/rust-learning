use reqwest;
use scraper::{Html, Selector};

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

    // HTML
    let resp = reqwest::get(TIEBA_URL).await?;
    //println!("Body:{:#?}",resp.text().await?);
    let body = resp.text().await?;
    // println!("body:{}",&body);
    let doc = Html::parse_fragment(&body);
    let selector = Selector::parse(".BDE_Image").unwrap();
    for el in doc.select(&selector) {
        println!("{}",el.value().attr("src").unwrap());
    }
    Ok(())
}