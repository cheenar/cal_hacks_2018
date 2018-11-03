use super::math;

pub trait FieldElement<Element = Self> {
	fn add(&self, other: &Element) -> Element;
	fn sub(&self, other: &Element) -> Element;
	fn mul(&self, other: &Element) -> Element;
	fn div(&self, other: &Element) -> Element;
}

pub trait Field<T: FieldElement> {
    fn zero(&self) -> impl T;
    fn identity(&self) -> impl T;
}

struct FiniteField<T> {
    modulus: T
}

impl Field<Gf32> for FiniteField<Gf32> {

}

// field modulo 2147473697
#[derive(Debug)]
struct Gf32 {
    value : u32,
}

const MODULUS_GF32 : u32 = 2147473697;
//const MODULUS_GF32 : u32 = 15;


impl FieldElement<Gf32> for Gf32 {
    fn add(&self, other: &Gf32) -> Gf32 {
		Gf32{
                value: (self.value + other.value) % MODULUS_GF32
            }
    }

    fn sub(&self, other: &Gf32) -> Gf32 {
        Gf32{
                value: (self.value - other.value) % MODULUS_GF32
            }
        }

    fn mul(&self, other: &Gf32) -> Gf32 {
        Gf32{
                value: (((self.value as u64) * (other.value as u64)) % (MODULUS_GF32 as u64)) as u32
            }
    }

    fn div(&self, other: &Gf32) -> Gf32 {
        let x = MODULUS_GF32 as i32;
        let y = other.value as i32;
        
        let (gcd, x_coeff, y_coeff) = math::egcd(x, y);

        //magic number
        //we only want the positive value returned from the GCD algorithm
        let magic: u32 = if x_coeff > y_coeff {
            x_coeff as u32
        }
        else {
            y_coeff as u32
        };

        println!("{} {} {}", gcd, x_coeff, y_coeff);
        Gf32{
            value: self.value * magic % MODULUS_GF32
        }
    }   
}

impl PartialEq for Gf32 {
    fn eq(&self, other: &Gf32) -> bool {
        self.value == other.value
    }
}