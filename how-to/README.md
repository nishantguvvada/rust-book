# HOW-TO GUIDE

## How to get user inputs in Rust?

- **Option 1: Use std::io**

```
use std::io;

fn main() {
    let mut input_value = String::new();

    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read.");

    println!("{}", input_value);
}
```

- **Option 2: Use std::env::args()**

```
fn main() {
    let secret_name = std::env::args().nth(1).expect("No secret name provided.");

    println!("{}", secret_name);
}

```

- **Option 3: Use clap::Parser**

```
use clap::Parser;
use clap_derive::Parser;

#[derive(Debug, Parser)]
struct Credentials {
    secret_name: String,
    secret_value: String
}

fn main() {
    let args = Credentials::parse();

    println!(
        "name: {:?}, value: {:?}",
        args.secret_name, args.secret_value
    )
}
```
