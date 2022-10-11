//! Count tester.
//!
//! A simple client that connects to a db-server and sends count expecting an integer response.
//!
//! You can test this out by running:
//!
//!     cargo run --bin db-server
//!
//! And then in another terminal run:
//!
//!     cargo run --example count

use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.

    // let host = "piedmont.local";
    let host = "127.0.0.1";
    let port = 6379;

    println!("connecing to {}:{}", host, port);

    let mut client = client::connect(format!("{}:{}", host, port)).await?;

    let result = client.count().await?;

    println!("count: {} from {}", result.count(), host);

    Ok(())
}
