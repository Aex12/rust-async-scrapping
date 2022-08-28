mod amazon;
use std::error::Error;

use reqwest::Client;

use amazon::parse as amazon_parse;
use crate::structs::Product;

pub struct Scraper {
    pub client: Client,
}

impl Scraper {
    pub async fn get (&self, url: &str) -> Result<Product, Box<dyn Error>> {
        let result = self.client.get(url).send()
            .await?
            .text()
            .await?;

        Ok(amazon_parse(result.as_str()))
    }
}
