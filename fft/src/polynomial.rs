use super::field;

pub struct Polynomial<T : field::FieldElement> {
    degree: usize,
    coefficients : Vec<T>,
}

impl Polynomial<T> {
    
    fn eval(&self, x: &T) {
        let mut sum = Gf32 { value: 0 };
        let mut curr_x = T.identity();
        for degree in 0 ..= self.degree {
            sum += self.coefficients[degree].value;
            curr_x = curr_x.mul(curr_x);
        }
    }

}    
                
fn eval_polynomial(poly: Vec<i32>, x: i32) -> i32 {
    let mut sum = 0;
    for degree in (0 .. poly.len()) {
        sum += poly[degree] * x.pow(degree as u32);
    }
    return sum;
}                                            