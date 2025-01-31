//mod strings;
mod float;
mod signed;
mod string;
mod unsigned;
// mod struct;

pub struct Calculator {
    first_value: u64,
    second_value: u64,
}

impl Calculator {
    pub fn addition(&self) -> u64 {
        self.first_value + self.second_value
    }
    pub fn subtract(&self) -> u64 {
        self.first_value - self.second_value
    }
    pub fn multiply(&self) -> u64 {
        self.first_value * self.second_value
    }
    pub fn division(&self) -> u64 {
        self.first_value / self.second_value
    }
}

fn main() {
    unsigned::intro_to_u();
    signed::intro_to_i();
    float::intro_to_float();
    string::strings();

    let calculation = Calculator {
        first_value: 70,
        second_value: 50,
    };

    let add = calculation.addition();
    let subtract = calculation.subtract();
    let division = calculation.division();
    let multiply = calculation.multiply();
    println!("addition {add}");
    println!("subtraction {subtract}");
    println!("division {division}");
    println!("multiplication {multiply}");
}
