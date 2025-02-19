#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
    author: &'a str,
    year: u128,
    likes: u16,
}

impl Book<'static> {
    pub fn new_book<'a>(title: &'a str, author: &'a str, year: u128) -> Book<'a> {
        Book {
            title,
            author,
            year,
            likes: 0,
        }
    }
}

pub fn book_registry() {
    let power = Book::new_book("power", "courtney A. kemp", 2014);
    println!("latest book {:#?}", power)
}
