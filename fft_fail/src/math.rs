pub fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        return (b, 0, 1);
    }
    else {
        let (g, x, y) = egcd(b % a, a);
        return (g, y - (b / a) * x, x);
    }
}