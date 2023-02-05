mod amazon;

use anyhow::Result;
use reqwest::Client;

use crate::client::get_client;
use crate::structs::Product;
use amazon::parse as amazon_parse;

use debug_print::debug_println as dprintln;

#[derive(Clone)]
pub struct Scraper {
    pub client: Client,
}

impl Scraper {
    pub async fn get(&self, url: &str) -> Result<Product> {
        dprintln!("getting: {url}");

        let result = self.client
            .get(url).send().await?
            .text().await?;

        dprintln!("finished get: {url}");

        amazon_parse(&result)
    }

    pub fn new() -> Result<Self> {
        Ok(Self {
            client: get_client()?,
        })
    }
}
