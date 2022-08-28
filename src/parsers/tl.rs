use crate::structs::{Selectors, Results};
use crate::utils::convert_to_number;

pub fn parse (input: &str, selectors: Selectors) -> Results {
    let dom = tl::parse(
        input,
        tl::ParserOptions::default()
    ).unwrap();

    let parser = dom.parser();

    let get = |selector: &str| -> String {
        let element = dom.query_selector(selector).unwrap()
            .next().unwrap();

        element.get(parser).unwrap().inner_text(parser).to_string()
    };

    return Results {
        title: get(selectors.title),
        image: get(selectors.image),
        date:  convert_to_number(get(selectors.date))
    };
}

