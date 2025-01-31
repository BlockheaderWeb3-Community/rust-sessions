
pub fn main(){
    struct Calc {
        num1: u32,
        num2: u32,
    }
    
    impl Calc {
        fn new(num1: u32, num2: u32) -> Self {
            Calc { num1, num2 }
        }
    
        fn add(&self) -> u32 {
            self.num1 + self.num2
        }
    
        fn subtract(&self) -> u32 {
            self.num1 - self.num2
        }
    
        fn multiply(&self) -> u32 {
            self.num1 * self.num2
        }
    
        fn divide(&self) -> Option<u32> {
            if self.num2 == 0 {
                None
            } else {
                Some(self.num1 / self.num2)
            }
        }
    
        fn modulus(&self) -> Option<u32> {
            if self.num2 == 0 {
                None
            } else {
                Some(self.num1 % self.num2)
            }
        }
    }

    let calculator = Calc::new(10, 5);
    println!("Addition: {}", calculator.add());
    println!("Subtraction: {}", calculator.subtract());
    println!("Multiplication: {}", calculator.multiply());
    match calculator.divide() {
        Some(result) => println!("Division: {}", result),
        None => println!("Cannot divide by zero"),
    }
    match calculator.modulus() {
        Some(result) => println!("Modulus: {}", result),
        None => println!("Cannot work with zero"),
    }
}