
fn main() {

    let m = 2.0;
    let c = 1.0;

    let lin = |x| m * x + c;

    println!("res {} {}", lin(1.0), lin(2.0));
}