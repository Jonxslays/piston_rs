use piston_rs::{Client, Executor, File};

// Here for testing purposes

#[tokio::main]
async fn main() {
    // Create a new mutable Client
    let client = Client::default();

    // // Fetch languages from Piston. Note: The languages are added onto the
    // // Client, and a reference to them is returned.
    // // If there was an error, print it and return from the main function.
    // if let Err(e) = client.fetch_languages().await {
    //     println!("{}", e);
    //     return;
    // }

    // Create a new File to send to Piston. This will contain our source code.
    let file = File::default().set_name("test.py").set_content(
        "import random
print(random.randint(1, 10))
print('Hello from python!')",
    );

    // Create a new executor, this represents the language, other metadata
    // about the code we are sending to Piston.
    let executor = Executor::default()
        .set_language("python")
        .set_version("3.10")
        .add_file(file);

    // Send the response to Piston, and store the result.
    let result = client.execute(&executor).await;

    if let Ok(data) = result {
        println!("{}", data.run.output);
    } else {
        println!("{}", result.unwrap_err());
    }
}
