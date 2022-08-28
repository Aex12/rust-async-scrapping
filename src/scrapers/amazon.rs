use anyhow::Result;

use crate::structs::{Product, Selectors};
use crate::parsers::tl::parse as tl_parse;

pub fn parse (data: &str) -> Result<Product> {
    let selectors = Selectors {
        title: "title",
        image: "#landingImage",
        price: "span.a-offscreen"
    };

    Ok(tl_parse(
        data,
        selectors
    )?)
}
