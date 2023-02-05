use anyhow::Result;
use debug_print::debug_println as dprintln;
use tokio::time::{sleep, Duration};

// use futures::{stream, StreamExt};

use html_parser::coordinator::Coordinator;


#[tokio::main]
async fn main() -> Result<()> {
    dprintln!("initialized");
    let coordinator = Coordinator::new().unwrap();

    loop {
        let urls = coordinator.get_tasks()?;
        for url in urls {
            coordinator.get(url).await;
        }
    }
}
