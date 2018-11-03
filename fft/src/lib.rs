// field modulo 2147473697
#[derive(Debug)]
struct Gf32 {
    value : u32,
}

const MODULUS_GF32 : u32 = 2147473697;

impl Gf32 {
    pub fn add(&self, a: Gf32) -> Gf32 {
        Gf32{
                value: (self.value + a.value) % MODULUS_GF32
            }
    }

    pub fn sub(&self, a: Gf32) -> Gf32 {
        Gf32{
                value: (self.value - a.value) % MODULUS_GF32
            }
    }

    pub fn mul(&self, a: Gf32) -> Gf32 {
        Gf32{
                value: (((self.value as u64) * (a.value as u64)) % (MODULUS_GF32 as u64)) as u32
            }
    }

    pub fn div(&self, a: Gf32) -> Gf32 {
        let x = MODULUS_GF32 as i32;
        let y = a.value as i32;
        
        let (gcd, x_coeff, y_coeff) = egcd(x, y);

        //magic number
        //we only want the positive value returned from the GCD algorithm
        let magic: u32 = if x_coeff > y_coeff {
            x_coeff as u32
        }
        else {
            y_coeff as u32
        };

        Gf32{
            value: self.value * magic % MODULUS_GF32
        }
    }   
}

fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        return (b, 0, 1);
    }
    else {
        let (g, x, y) = egcd(b % a, a);
        return (g, y - (b / a) * x, x);
    }
}

fn naive_fft(poly: Vec<Gf32>, eval_coords: Vec<Gf32>) -> Vec<Gf32> {
    let evals: Vec<Gf32> = Vec::new();
    for x in eval_coords.iter() {
        let mut xPow = Gf32{value: 1};
        let mut eval = Gf32{value: 0};
        for coeff in poly.iter() {
            eval = eval.add(xPow.mul(*coeff));
            xPow = xPow.mul(*x);
        }
    }
    return evals;
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
    fn div() {
        let x = Gf32{value: 1};
        let y = Gf32{value: 53063};
        let z = Gf32{value: 691636837};
        //println!("{}", x.div(y).value);
        assert_eq!(x.div(y), z);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}