// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

// generic type implementation but does
// not include refrence type

// pub trait Power<T> {
//     fn power(&self,other:T)-> u32;
// }

// impl<T:Into<u32>>  Power<T> for  u32 {
//     fn power(&self,other:T)-> u32 {
//         let right = other.into();
//         self.pow(right)
//     }
// }

pub trait Power<Rhs = Self> {
    fn power(&self, other: Rhs) -> u32;
}

impl Power<u16> for u32 {
    fn power(&self, other: u16) -> u32 {
        let right = other as u32;
        self.pow(right)
    }
}

impl Power<u32> for u32 {
    fn power(&self, other: u32) -> u32 {
        let right = other as u32;
        self.pow(right)
    }
}

impl Power<&u32> for u32 {
    fn power(&self, other: &u32) -> u32 {
        let right = *other as u32;
        self.pow(right)
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
