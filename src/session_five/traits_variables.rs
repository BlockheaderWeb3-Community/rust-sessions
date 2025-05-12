pub trait Drive {
    fn print_driving(&self) {
        println!(" i have started driving");
    }
}

pub struct Truck {
    pub capacity: u32,
    pub serial_number: u32,
}

impl Drive for Truck {
    fn print_driving(&self) {
        println!(" i have started driving the truck");
    }
}

pub fn trait_variables() {
    // this would give erro because the dynamic trait size cannot be known at compile time
    // let x: dyn Drive;

    // use the box smart pointer to store it on the heap
    let x: Box<dyn Drive>;

    let honda = Truck {
        capacity: 200,
        serial_number: 40444,
    };

    x = Box::new(honda);

    x.print_driving();
}
