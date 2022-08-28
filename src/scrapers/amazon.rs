use crate::structs::{Product, Selectors};
use crate::parsers::tl::parse as tl_parse;

pub fn parse (data: &str) -> Product {
    let selectors = Selectors {
        title: "title",
        image: "img[src]",
        price: "span.a-offscreen"
    };

    tl_parse(
        data,
        selectors
    )
}
