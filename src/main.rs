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
    ).await;

    match result {
        Ok(v) => print!("title: {}\r\nimage: {}\r\n price: {}\r\n", v.title, v.image, v.price),
        Err(e) => print!("Error: {}\r\n", e),
    }

    Ok(())
}
