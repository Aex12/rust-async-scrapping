use anyhow::Result;

use debug_print::debug_println as dprintln;

use tokio::sync::Semaphore;
use crate::scrapers::Scraper;

pub struct Coordinator {
    semaphore: Semaphore,
    scraper: Scraper,
}

impl Coordinator {
    pub async fn get_tasks(&self) -> Result<Vec<&str>> {
        Ok(vec![
            "https://www.amazon.es/dp/B08YRVM51Q",
            "https://www.amazon.es/dp/B07TN2RX2K",
            "https://www.amazon.es/dp/B07LCQLC8Y",
            "https://www.amazon.es/dp/B09HSQQWCL",
        ])
    }

    pub async fn get(&self, url: &str) -> Result<()> {
        dprintln!("added to queue: {url}");
        let permit = self.semaphore.acquire().await.unwrap();
        dprintln!("getting: {url}");

        // let scraper = self.scraper.clone();
        tokio::spawn(async move {
            // let product = scraper.get(url).await?;

            drop(permit);
            // dprintln!("{product}");
        });

        Ok(())
    }

    pub fn new() -> Result<Self> {
        Ok(Self {
            semaphore: Semaphore::new(1),
            scraper: Scraper::new()?,
        })
    }
}
