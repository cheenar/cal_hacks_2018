use num_bigint::BigUInt;

pub struct FiniteField<T : BigUInt> {
    modulus: T
}

impl<T : BigUInt> FiniteField<T> {
    fn add(&self, a: T, b: T) -> T {
        (a + b) % self.modulus
    }
}

#[cfg(test)]
mod tests {
    use super::FiniteField;
    #[test]
    fn add_test() {
        let x = FiniteField{modulus: 15 as i32};
        println!(x.add(4, 12))
    }
}
