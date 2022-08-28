mod client;

use std::error::Error;

use html_parser::scrapers::Scraper;
use client::get_client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let scraper = Scraper {
        client: get_client()?
    };

    let result = scraper.get(
        "https://www.amazon.es/quiet-Disipador-calor-calefactor-m%C3%B3dulos/dp/B08YRVM51Q"
    ).await?;

    print!("title: {}, image: {}, price: {}", result.title, result.image, result.price);

    Ok(())
}
