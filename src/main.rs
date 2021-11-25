use piston_rs::Client;

// Here for testing purposes

#[tokio::main]
async fn main() {
    let mut client = Client::default();
    match client.fetch_languages().await {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    println!("{:?}", client);
}
