use piston_rs::Client;

// Here for testing purposes

#[tokio::main]
async fn main() {
    let client = Client::default();
    match client.get_languages().await {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    println!("{:?}", client);
}
