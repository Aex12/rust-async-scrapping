mod amazon;

use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;

use tokio::sync::Semaphore;

use crate::client::get_client;
use crate::structs::Product;
use amazon::parse as amazon_parse;

use debug_print::debug_println as dprintln;

#[derive(Clone)]
pub struct Scraper {
    pub client: Client,
    semaphore: Arc<Semaphore>,
}

impl Scraper {
    pub async fn get(&self, url: &str) -> Result<Product> {
        dprintln!("added to queue: {url}");
        let permit = self.semaphore.acquire().await.unwrap();
        dprintln!("getting: {url}");

        let result = self.client
            .get(url).send().await?
            .text().await?;

        drop(permit);
        dprintln!("finished get: {url}");

        amazon_parse(&result)
    }

    pub async fn wait(&self) {
        dprintln!("awaiting for free semaphore");
        let permit = self.semaphore.acquire().await.unwrap();
        dprintln!("semaphore is free");
        drop(permit);
    }

    pub fn new() -> Result<Self> {
        Ok(Self {
            client: get_client()?,
            semaphore: Arc::new(Semaphore::new(1)),
        })
    }
}
