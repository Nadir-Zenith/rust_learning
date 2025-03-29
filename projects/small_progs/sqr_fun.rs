
fn sqr(x: f64) -> f64 {
    return x * x;
}

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}


// ensure the number always falls in the given range
fn clamp(x: f64, x1: f64, x2:f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

// recursive factorial
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}


// passing values by reference
fn by_ref(x: &i32) -> i32{
    *x + 1
}

// mutable references 
fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let mut res = sqr(2.0);
    modifies(&mut res);
    println!("square is {}", res);
}