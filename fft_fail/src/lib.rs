extern crate num_bigint;


pub mod field;
pub mod math;
//mod polynomial;

/*
#[cfg(test)]
mod tests {
    use super::field::Gf32;

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
        assert_eq!(x.div(y), z);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/