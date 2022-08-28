use crate::structs::{Selectors, Product};
use crate::utils::convert_to_number;

pub fn parse (input: &str, selectors: Selectors) -> Product {
    let dom = tl::parse(
        input,
        tl::ParserOptions::default()
    ).unwrap();

    let parser = dom.parser();

    let get = |selector: &str| -> &tl::Node {
        dom.query_selector(selector).unwrap()
            .next().unwrap()
            .get(parser).unwrap()
    };

    let get_text = |selector: &str| -> String {
        get(selector)
            .inner_text(parser).to_string()
    };

    let get_attribute = |selector: &str, attr: &str| -> String {
        get(selector)
            .as_tag().unwrap()
            .attributes().get(attr).flatten().unwrap()
            .try_as_utf8_str().unwrap()
            .to_string()
    };

    return Product {
        title: get_text(selectors.title),
        image: get_attribute(selectors.image, "src"),
        price:  convert_to_number(get_text(selectors.price))
    };
}

