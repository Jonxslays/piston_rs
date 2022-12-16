<h1 align="center">piston_rs</h1>
<p align="center">
<a href="https://crates.io/crates/piston_rs"><img height="20" alt="Crate" src="https://img.shields.io/crates/v/piston_rs"></a>
<a href="https://crates.io/crates/piston_rs"><img height="20" alt="Docs" src="https://img.shields.io/docsrs/piston_rs"></a>
<a href="https://github.com/Jonxslays/piston_rs/actions/workflows/ci.yml"><img height="20" alt="Build" src="https://img.shields.io/github/actions/workflow/status/Jonxslays/piston_rs/ci.yml?branch=master"></a>
</p>

<p align="center">An async wrapper for the <a href="https://github.com/engineer-man/piston">Piston</a> code execution engine.</p>

## Why piston_rs

piston_rs aims to make interacting with Piston fun and easy. Your main
tools are the [`Client`](https://docs.rs/piston_rs/latest/piston_rs/struct.Client.html)
and [`Executor`](https://docs.rs/piston_rs/latest/piston_rs/struct.Executor.html) structs.

The [`Executor`](https://docs.rs/piston_rs/latest/piston_rs/struct.Executor.html)
is constructed containing the source code and other metadata about the code you are
running. This is then sent to Piston via the
[`Client`](https://docs.rs/piston_rs/latest/piston_rs/struct.Client.html).

piston_rs requires Rust version 1.46.0 or greater.

## Getting started

For more details, check out the [documentation](https://docs.rs/piston_rs/latest)!

### Add piston_rs to your project

```toml
# Cargo.toml

[dependencies]
piston_rs = "^0.4"
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
                .set_content("fn main() { println!(\"42\"); }")
        );

    match client.execute(&executor).await {
        Ok(response) => {
            println!("Language: {}", response.language);
            println!("Version: {}", response.version);

            if let Some(c) = response.compile {
                println!("Compilation: {}", c.output);
            }

            println!("Output: {}", response.run.output);
        }
        Err(e) => {
            println!("Something went wrong contacting Piston.");
            println!("{}", e);
        }
    }
}
```

## License

piston_rs is licensed under the [MIT License](https://github.com/Jonxslays/piston_rs/blob/master/LICENSE).
