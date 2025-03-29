fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "hello dolly";  //the string slice
    let s = test.to_string();  // it's now an allocated string

    dump(text);
    dump(&s);
}