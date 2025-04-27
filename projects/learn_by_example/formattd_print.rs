fn main() {
    // in general, the `{}` will be automatically replaced with any 
    // arguments. These will be stringified.
    println!("{} days",31);

    // positional arguments can be used. Specifying an integer inside `{}`
    // determines with additional arguments will be replaced. Arguments start
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // as can named arguments.
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
        );

    // different number bases
    println!("Base 10:  {}", 532);
    println!("Base 2 (binary): {:b}", 532);
    println!("Base 8 (octal):  {:o}", 532);
    println!("Base 16 (hexadecimal): {:x}", 532);

    // justify
    println!("{number:>5}", number=1);

    // padding
    println!("{number:0>5}", number=1); 
    println!("{number:0<5}", number=1);

    // named arguments
    println!("{number:0>width$", number=1, width=5);

}