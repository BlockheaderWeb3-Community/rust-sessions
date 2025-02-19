#[derive(Debug)]
#[warn(dead_code)]
pub struct Book<'a> {
    // 1. Define the fields of the struct
    // Make all of them public with `pub`
    // Read the description for the fields
    pub title: &'a str,
    pub author: &'a str,
    pub year: i64,
    pub likes: u16,
}

pub fn book_new() {
    let book = Book::new("Get it right", "martin luther vibes", 2026);
    println!("this is the result of the new book {:#?}", book)
}

impl Book<'static> {
    // 2. Define the `new` associated function
    pub fn new<'a>(title: &'a str, author: &'a str, year: i64) -> Book<'a> {
        Book {
            title,
            author,
            year,
            likes: 0,
        }
    }
}
