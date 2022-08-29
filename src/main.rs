use anyhow::Result;
use debug_print::debug_println as dprintln;

use futures::{stream, StreamExt};
use tokio;

use html_parser::scrapers::Scraper;

const CONCURRENT_REQUESTS: usize = 2;

#[tokio::main]
async fn main() -> Result<()> {
    dprintln!("initialized");
    let scraper = Scraper::new()?;

    let urls = [
        "https://www.amazon.es/dp/B08YRVM51Q",
        "https://www.amazon.es/dp/B07TN2RX2K",
        "https://www.amazon.es/dp/B07LCQLC8Y",
        "https://www.amazon.es/dp/B09HSQQWCL"
    ];

    let results = stream::iter(urls)
        .map(|url| {
            let scraper = &scraper;
            async move {
                scraper.get(url).await
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    results
        .for_each(|result| async {
            match result {
                Ok(v) => print!("{}", v),
                Err(e) => print!("Error: {}\r\n", e),
            }
        })
        .await;

    Ok(())
}
