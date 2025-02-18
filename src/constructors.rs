pub struct Book {
    pub title: String,
    pub author: String, 
    pub year: u32,
    pub likes: u32,


}

impl Book {
fn new(title: String, author: String, year: u32) -> Book {
   Book {
        title,
        author,
        year,
        likes: 0
    }

}
// 2. Define the `new` associated function
}

fn main() {
let new_book = Book {
    title:"The Hidden Ember Ignited the Sky"to_string(),
    author:"Jemiiah".to_string ,
    year: 2027,
    likes: 0,
};

}