use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use html_parser::parsers::tl::parse as tl_parse;
use html_parser::structs::Selectors;

const SELECTORS: Selectors = Selectors {
    title: "title",
    image: "img[src]",
    price: "span.a-offscreen"
};

fn criterion_benchmark(c: &mut Criterion) {
    let data = fs::read_to_string("website.html")
        .expect("Unable to read html file");


    c.bench_function("tl", |b| b.iter(|| black_box(tl_parse(&data.as_str(), SELECTORS))));
    // c.bench_function("fib 30", |b| b.iter(|| tl(black_box(data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
