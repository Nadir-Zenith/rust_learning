# Debug

Printing in debug mode.
```rust
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);


fn main() {
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
                                "Slater", 
                                "Christian",
                                 actor="actor's");

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will pring!", Deep(Structure(7)));
}
```

Rust also provides "pretty printing" with `{:#?}`.

```rust
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
```