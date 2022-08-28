use anyhow::{Result, Context};

use crate::structs::{Selectors, Product};
use crate::utils::convert_to_number;

pub fn parse (input: &str, selectors: Selectors) -> Result<Product>{
    let dom = tl::parse(
        input,
        tl::ParserOptions::default()
    )?;

    let parser = dom.parser();

    let get = |selector: &str| -> Result<&tl::Node> {
        Ok(
            dom.query_selector(selector).context("Failed to run query selector")?
                .next().context("Failed to find an element matching query selector")?
                .get(parser).context("Parser failed to get element from query selector")?
        )
    };

    let get_text = |selector: &str| -> Result<String> {
        Ok(get(selector)?
            .inner_text(parser).to_string())
    };

    let get_attribute = |selector: &str, attr: &str| -> Result<String> {
        Ok(get(selector)?
            .as_tag().context("Couldn't convert to tag")?
            .attributes().get(attr).flatten().context("Couldnt get attribute")?
            .try_as_utf8_str().context("Couldn't convert to utf8 str")?
            .to_string())
    };

    Ok(Product {
        title: get_text(selectors.title)?,
        image: get_attribute(selectors.image, "src")?,
        price: convert_to_number(get_text(selectors.price)?)
    })
}

