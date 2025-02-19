```
pub struct Book<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub year: u32,
    pub likes: u32,
}

impl<'a> Book<'a> {
    pub fn new(title: &'a str, author: &'a str, year: u32) -> Book<'a> {
        Book {
            title,
            author,
            year,
            likes: 0,
        }
    }
}
```
