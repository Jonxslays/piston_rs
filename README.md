# piston_rs

An async wrapper for the [Piston](https://github.com/engineer-man/piston)
code execution engine.

## Why piston_rs

piston_rs aims to make interacting with Piston fun and easy. Your main
tools are the `Client` and `Executor` structs.

The `Executor` is constructed containing the source code and other
metadata about the code you are running. This is then sent to Piston
via the `Client`.

piston_rs requires Rust version 1.46.0 or greater.

## Getting started

For more details, check out the [documentation](https://crates.io/crates/piston_rs)!

### Add piston_rs to your project

```toml
# Cargo.toml

[dependencies]
piston_rs = "^0.2"
```

### Make requests to Piston

```rs
// main.rs

#[tokio::main]
async fn main() {
    let client = piston_rs::Client::new();
    let executor = piston_rs::Executor::new()
        .set_language("rust")
        .set_version("*")
        .add_file(
            piston_rs::File::default()
                .set_name("main.rs")
                .set_content("fn main() { println!(\"42\"); }"),
        );

    match client.execute(&executor).await {
        Ok(response) => {
            if response.is_err() {
                println!("{}", response.message.unwrap());
            } else {
                println!("Language: {}", response.language);
                println!("Version: {}", response.version);
                println!("Output:\n{}", response.run.output);
            }
        },
        Err(e) => {
            println!("Something went wrong contacting Piston.");
            println!("{}", e);
        },
    }
}
```

## License

piston_rs is licensed under the [MIT License](https://github.com/Jonxslays/piston_rs/blob/master/LICENSE).
