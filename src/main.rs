use reqwest;
use scraper::{Html, Selector};
use std::io::{BufRead, BufReader, Write};
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
    thread, time,
};
use tokio;

// Sleeps for m amount of milliseconds
fn sleep(m: u64) {
    let sleep_time = time::Duration::from_millis(m);
    thread::sleep(sleep_time);
}

// Create folder and individual HTML file for each comic count
fn create_cache(count: u64) -> PathBuf {
    let mut path = PathBuf::from("./cache");
    if !path.exists() {
        fs::create_dir(&path).expect("Failed to create directory 'cache'");
    }
    path.push(format!("{}.html", count));

    path
}

fn append_to_cache(count: u64, item: String) {
    let path = create_cache(count);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .expect("Failed to open html file for appending");

    writeln!(file, "{}", item).expect("Failed to write to html file");
}

fn get_cache(count: u64) -> String {
    let path = PathBuf::from(format!("./cache/{}.html", count));

    if path.exists() {
        let file = OpenOptions::new()
            .read(true)
            .open(&path)
            .expect("Failed to open cache.html for reading");

        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(comic) => return comic,
                Err(e) => println!("Failed to read line: {}", e),
            }
        }
    }
    "no comic found".to_string()
}

async fn parse_html(url: &str) -> Html {
    let html = reqwest::get(url)
        .await
        .expect("Failed to get response.")
        .text()
        .await
        .expect("Failed to convert response into text.");

    Html::parse_document(&html)
}

fn get_comic_img(html: &Html) -> String {
    let selector = Selector::parse("img#cc-comic").unwrap();
    for element in html.select(&selector) {
        if let Some(src) = element.value().attr("src") {
            return src.to_string();
        }
    }
    "no comic img".to_string()
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    const AMOUNT: u64 = 15;
    let mut url = String::from("https://www.smbc-comics.com/comic/wishes-6");

    for i in 1..=AMOUNT {
        
        println!("count: {}", i);

        let cached = get_cache(i);
        let comic_img = if cached == "no comic found" {
            sleep(555);
            let html = parse_html(&url).await;
            let img = get_comic_img(&html);
            append_to_cache(i, img.clone());

            // Update to previous comic URL
            let selector = Selector::parse("a.cc-prev").unwrap();
            for element in html.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    url = format!("{}", href);
                    break;
                }
            }

            img
        } else {
            cached + "!!!! from cache"
        };

        println!("Cached entry {}: {}", i, comic_img);
        println!("Finished looking at {}\n", url);
    }

    Ok(())
}