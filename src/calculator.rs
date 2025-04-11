use std::ops::{Add, Div, Mul, Sub};

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

pub trait Power {
    fn powf(self, exponent: Self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Power for f32 {
    fn powf(self, exponent: Self) -> Self {
        self.powf(exponent)
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Power for f64 {
    fn powf(self, exponent: Self) -> Self {
        self.powf(exponent)
    }
}

pub fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn subtract<T: Sub<Output = T>>(a: T, b: T) -> T {
    a - b
}

pub fn multiply<T: Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

pub fn division<T: Div<Output = T> + PartialEq + Default>(a: T, b: T) -> T {
    if b == T::default() {
        panic!("Division by zero is not allowed");
    }
    a / b
}

pub fn square_root<T: Sqrt>(a: T) -> T {
    a.sqrt()
}

pub fn power<T: Power>(base: T, power: T) -> T {
    base.powf(power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        assert_eq!(power(2.0, 8.0), 256.0);
    }

    #[test]
    fn test_square_root() {
        assert_eq!(square_root(64.0), 8.0);
    }

    #[test]
    fn test_add_float() {
        assert_eq!(add(12.0, 13.0), 25.0);
    }

    #[test]
    fn test_add_int() {
        assert_eq!(add(45, 21), 66);
    }

    #[test]
    fn test_subtract() {
        // assert_eq!(subtract(a, b))
        let sum: f64 = subtract(12.0, 6.0);

        assert!(sum == 6.0, "Value should be 6.0");
    }

    #[test]
    fn test_sub() {
        let sum: f64 = subtract(12.0, 6.1);

        assert!(sum == 5.9, "Value should be 6.0");
        // assert_eq!()
        //
        assert_eq!(subtract(12.0, 6.1), 5.9);
    }

    #[test]
    fn test_sub_int() {
        let sub = subtract(12, 6);
        assert_eq!(subtract(12, 6), 6);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(8.0, 8.0), 64.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(division(12.0, 6.0), 2.0);
    }

    #[test]
    #[should_panic(expected = "Division by zero is not allowed")]
    fn test_division1() {
        assert_eq!(division(12.0, 0.0), 0.0);
    }
}
