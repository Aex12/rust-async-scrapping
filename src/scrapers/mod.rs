mod amazon;

use reqwest::Client;
use anyhow::Result;

use amazon::parse as amazon_parse;
use crate::structs::Product;
use crate::client::get_client;

use debug_print::debug_println as dprintln;

#[derive(Clone)]
pub struct Scraper {
    pub client: Client,
}

impl Scraper {
    pub async fn get (&self, url: &str) -> Result<Product> {
        dprintln!("getting: {url}");

        let result = self.client.get(url).send()
            .await?
            .text()
            .await?;

        Ok(amazon_parse(&result)?)
    }

    pub fn new () -> Result<Scraper> {
        Ok(Scraper {
            client: get_client()?,
        })
    }
}

