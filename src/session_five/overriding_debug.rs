use std::ops::Deref;

pub fn overriding_debug() {
    struct SessionFive {
        x: String,
        y: String,
    }

    let mysession = SessionFive {
        x: String::from("4"),
        y: String::from("7"),
    };

    impl Deref for SessionFive {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.x
        }
    }

    // over ridding the deref trait will print out the x value of the struct
    println!(" this is my session and my session is this {}", *mysession);
}
