# The very first chapter

```rust
# fn main() {
    let number = 5;
    println!("{}", number);
# }
```

\begin{cases} \frac 1 2 \\\\ \frac 3 4 \end{cases}


```python, hidelines=!!!
!!!hidden()
nothidden()
```

```rust,noplayground
let mut name = String::new();
std::io::stdin().read_line(&mut name).expect("failed to read line");
println!("Hello {}!", name);
```

Here is a component:
```rust,no_run,noplayground
{{#include file.rs:component}}
```

Here is a system:
```rust,no_run,noplayground
{{#include file.rs:system}}
```

This is the full file.
```rust,no_run,noplayground
{{#include file.rs:all}}
```

```rust,editable
# contents of example.rs here.
```

<img class="right" src="images/rust-logo-blk.svg" alt="The Rust logo">


<div class="warning">

This is a bad thing that you should pay attention to.

Warning blocks should be used sparingly in documentation, to avoid "warning
fatigue," where people are trained to ignore them because they usually don't
matter for what they're doing.

</div>
