use anyhow::Result;
use std::time::Duration;

use reqwest::{header, Client};

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.102 Safari/537.36";

pub fn get_client() -> Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static(USER_AGENT));
    Ok(Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(5))
        .default_headers(headers)
        .build()?)
}
