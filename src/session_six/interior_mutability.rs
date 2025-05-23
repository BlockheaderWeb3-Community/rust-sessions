use std::cell::{Cell, RefCell, UnsafeCell};

struct Counter {
    value: Cell<u32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: Cell::new(0),
        }
    }

    fn increment(&self) {
        self.value.set(self.value.get() + 1);
    }

    fn get(&self) -> u32 {
        self.value.get()
    }

    fn increment_by_number(&self, number: u32) {
        self.value.set(self.value.get() + number)
    }
}

struct Database {
    data: RefCell<Vec<String>>,
}

impl Database {
    fn new() -> Self {
        Database {
            data: RefCell::new(Vec::new()),
        }
    }

    fn insert(&self, item: String) {
        self.data.borrow_mut().push(item);
    }

    fn items(&self) -> Vec<String> {
        self.data.borrow().clone()
    }
}

pub struct MyCell<T> {
    value: UnsafeCell<T>,
}

impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> &T {
        // Safety: We're only returning an immutable reference
        unsafe { &*self.value.get() }
    }

    pub fn set(&self, new_value: T) {
        // Safety: We're replacing the value entirely
        unsafe {
            *self.value.get() = new_value;
        }
    }
}

pub fn main() {
    let number = Counter::new();
    number.get();
    println!("{}", number.get());
    number.increment();
    println!("{}", number.get());
    number.increment_by_number(4);
    println!("{}", number.get());

    let db = Database::new();
    db.insert("Data 1".to_string());
    db.insert("Data 2".to_string());

    let items = db.items();
    println!("Items: {:?}", items); // Output: Items: ["Data 1", "Data 2"]

    // MyCell
    let number = MyCell::new(20);
    number.get();
    println!("{}", number.get());
}
