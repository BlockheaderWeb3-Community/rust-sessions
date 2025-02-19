#[derive(Debug)]
pub struct Book<'a> {
    pub title: &'a str,
    pub author: &'a str, 
    pub year: i32,
    pub likes: i32,


}

impl<'a> Book<'a> {
pub fn new(title: &'a str, author: &'a str, year: i32) -> Book<'a> {
   Book {
        title,
        author,
        year,
        likes: 0
    }

}

}

pub fn constructors () {
let new_book = Book::new (
    "The Hidden Ember Ignited the Sky",
    "Jemiiah",
    2027,
);
println!("the new_book overview is {:?}", new_book);
}