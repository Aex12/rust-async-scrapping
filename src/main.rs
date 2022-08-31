use anyhow::Result;
use debug_print::debug_println as dprintln;
use tokio::time::{sleep, Duration};

// use futures::{stream, StreamExt};

use html_parser::scrapers::Scraper;


#[tokio::main]
async fn main() -> Result<()> {
    dprintln!("initialized");
    let scraper = Scraper::new()?;

    let urls = [
        "https://www.amazon.es/dp/B08YRVM51Q",
        "https://www.amazon.es/dp/B07TN2RX2K",
        "https://www.amazon.es/dp/B07LCQLC8Y",
        "https://www.amazon.es/dp/B09HSQQWCL",
    ];

    loop {
        scraper.wait().await;
        for url in urls {
            let scraper = scraper.clone();
            tokio::spawn(async move {
                let result = scraper.get(url).await;
                match result {
                    Ok(v) => print!("{}", v),
                    Err(e) => eprintln!("Error: {}", e),
                }
            });
            sleep(Duration::from_millis(100)).await;
        }
    }
}
