use anyhow::Result;

use crate::parsers::tl::parse as tl_parse;
use crate::structs::{Product, Selectors};

pub fn parse(data: &str) -> Result<Product> {
    let selectors = Selectors {
        title: "title",
        image: "#landingImage",
        price: "span.a-offscreen",
    };

    tl_parse(data, selectors)
}
