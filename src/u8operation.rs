pub fn sum(x: u8, y: u8) -> u8 {
    x + y // implicit return
          //    return x + y; // explicit return
}



// subtract

pub fn subtract(x: u8, y: u8) -> u8 {
    return x - y;
}

// multiplication

 pub fn multiply(x: u8, y: u8) -> u8 {
    x * y
}
// division

 pub fn divide(x: u8, y: u8) -> u8 {
    x / y
}

// function to check for even numbers

pub fn is_even(x: u8) -> bool {
    x % 2 == 0
}

// to check for odd nummbers

pub fn is_odd(x: u8) -> bool {
    x % 2 != 0
}

