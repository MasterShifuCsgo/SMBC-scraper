use reqwest;
use scraper::{Html, Selector};
use std::{fs, thread, time};
use tokio;

//sleeps for m amount of milliseconds
fn sleep(m: u64) {
    let sleep_time = time::Duration::from_millis(m);
    thread::sleep(sleep_time);
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // comic: https://smbc-comics.com/
    let amount = 3; // how many times a new comic should be fetched

    let mut url: String = String::from("https://www.smbc-comics.com/comic/wishes-6");

    for _ in 0..amount {
        sleep(1000);                
        
        let html = reqwest::get(url.clone())
            .await?
            .text()
            .await?;

        print!("New url Returned\n");
        let html = Html::parse_document(&html);

        
        //getting the next button of the webpage
        let selector = Selector::parse("a.cc-prev").unwrap();
        for element in html.select(&selector) {
            
            if let Some(href) = element.value().attr("href") {
                println!("prev button link: {}", href);
                url = href.to_string();
            }
        }

        //getting comic picture
        

    }
    Ok(())
}
