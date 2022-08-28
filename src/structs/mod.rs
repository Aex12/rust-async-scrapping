pub struct Selectors <'a> {
    pub title: &'a str,
    pub image: &'a str,
    pub price: &'a str
}

pub struct Product {
    pub title: String,
    pub image: String,
    pub price: u32
}
