use std::fs;
use std::error::Error;

use html_parser::parsers::tl::parse as tl_parse;
use html_parser::structs::Selectors;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("website.html")
        .expect("Unable to read html file");

    let selectors = Selectors {
        title: "title",
        image: "img[src]",
        date: "span.a-offscreen"
    };

    let result = tl_parse(
        data.as_str(),
        selectors
    );

    print!("title: {}, image: {}, date: {}", result.title, result.image, result.date);

    Ok(())
}
