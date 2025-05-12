use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn reference_count() {
    // Create a new Rc holding a List value
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // Clone the Rc, which increments the reference count
    let b = Cons(3, Rc::clone(&a)); // a's reference count = 2

    // Clone again
    let c = Cons(4, Rc::clone(&a)); // a's reference count = 3

    // When b and c go out of scope, the reference count decreases
    // Only when the count reaches 0 will the data be cleaned up
}
