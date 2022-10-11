//! Hello world client test.
//!
//! A simple client that connects to a mini-redis server, sets key "hello" with value "world",
//! and gets it from the server after
//!
//! You can test this out by running:
//!
//!     cargo run --bin db-server
//!
//! And then in another terminal or machine run:
//!
//!     cargo run --example hello_world

use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.

    let host = "piedmont.local";
    let port = 6379;

    println!("connecing to {}:{}", host, port);

    let mut client = client::connect(format!("{}:{}", host, port)).await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    if result.is_some() {
        println!("got value: {:?} from {}", result.unwrap(), host);
    } else {
        println!("{} returned success={:?}", host, result.is_some());
    }

    Ok(())
}
