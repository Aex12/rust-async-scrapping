use std::fmt;

pub struct Selectors<'a> {
    pub title: &'a str,
    pub image: &'a str,
    pub price: &'a str,
}

pub struct Product {
    pub title: String,
    pub image: String,
    pub price: u32,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
            title: {}\n\
            img: {}\n\
            price: {}\n\
        ",
            self.title, self.image, self.price
        )
    }
}
