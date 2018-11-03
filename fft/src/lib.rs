// field modulo 2147473697
#[derive(Debug)]
struct Gf32 {
    value : u32,
}
const MODULUS_GF32 : u32 = 2147473697;

impl Gf32 {
    fn add(&self, a: Gf32) -> Gf32 {
        Gf32{
                value: (self.value + a.value) % MODULUS_GF32
            }
    }

    fn sub(&self, a: Gf32) -> Gf32 {
        Gf32{
                value: (self.value - a.value) % MODULUS_GF32
            }
    }

    fn mul(&self, a: Gf32) -> Gf32 {
        Gf32{
                value: (((self.value as u64) * (a.value as u64)) % (MODULUS_GF32 as u64)) as u32
            }
    }
}

impl PartialEq for Gf32 {
    fn eq(&self, other: &Gf32) -> bool {
        self.value == other.value
    }
}

#[cfg(test)]
mod tests {
    use super::Gf32;
    #[test]
    fn add() {
        let x = Gf32{value: 1};
        let y = Gf32{value: 1};
        let z = Gf32{value: 2};
        assert_eq!(x.add(y), z);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}