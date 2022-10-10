//! Ping tester.
//!
//! A simple client that connects to a db-server and sends ping expecting a `pong` response.
//!
//! You can test this out by running:
//!
//!     cargo run --bin mini-redis-server
//!
//! And then in another terminal run:
//!
//!     cargo run --example ping

use mini_redis::{client, Result};
use std::str;

#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.

    let host = "piedmont.local";
    let port = 6379;

    println!("connecing to {}:{}", host, port);

    let mut client = client::connect(format!("{}:{}", host, port)).await?;

    let result = client.ping(None).await?;

    if let Ok(s) = str::from_utf8(&result) {
        println!("got value: {} from {}", s, host); 
    } else {
        println!("{} returned success={:?}", host, result);
    }

    Ok(())
}
