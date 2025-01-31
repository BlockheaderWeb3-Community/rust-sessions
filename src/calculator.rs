struct Calculator {
    value: f64,
}

impl Calculator {
    fn new() -> Calculator {
        Calculator {
            value: 5.0
        }
    }

    fn add(&mut self, number: f64) -> f64 {
        self.value += number;
        self.value
    }

    fn subtract(&mut self, number: f64) -> f64 {
        self.value -= number;
        self.value
    }

    fn multiply(&mut self, number: f64) -> f64 {
        self.value *= number;
        self.value
    }

    
    fn divide(&mut self, number: f64) -> Result<f64, String> {
        if number == 0.0 {
            return Err(String::from("Cannot divide by zero!"));
        }
        self.value /= number;
        Ok(self.value)
    }
    
}



pub fn cal() {
    let mut calc = Calculator::new();
    
    println!("Adding 5: {}", calc.add(10.0));
    println!("Multiplying by 2: {}", calc.multiply(2.0));
    println!("Subtracting 3: {}", calc.subtract(3.0));
    
    
    match calc.divide(2.0) {
        Ok(result) => println!("Dividing by 2: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}

