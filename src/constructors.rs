pub struct Book {
    pub title: &'a str,
    pub author: &'a str, 
    pub year: u32,
    pub likes: u32,


}

impl Book {
fn new(title: &'a str, author: &'a str , year: u32) -> Book {
   Book {
        title,
        author,
        year,
        likes: 0
    }

}

}



}