fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}


fn main() {
    let t = add_mul(2.0, 10.0);

    // can debug print
    println!("t {:?}", t);

    // can _extract_ values
    let (add, mul) = t;

    println!("add {}  mul {}", add, mul);
}