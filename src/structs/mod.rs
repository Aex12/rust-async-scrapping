pub struct Selectors <'a> {
    pub title: &'a str,
    pub image: &'a str,
    pub date: &'a str
}

pub struct Results {
    pub title: String,
    pub image: String,
    pub date: u32
}
