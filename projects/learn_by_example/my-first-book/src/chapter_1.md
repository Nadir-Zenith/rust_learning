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