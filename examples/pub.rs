//! Publish to a redis channel example.
//!
//! A simple client that connects to a mini-redis server, and
//! publishes a message on `foo` channel
//!
//! You can test this out by running:
//!
//!     cargo run --bin mini-redis-server
//!
//! Then in another terminal run:
//!
//!     cargo run --example sub
//!
//! And then in another terminal run:
//!
//!     cargo run --example pub

// #![warn(rust_2018_idioms)]

use domain_keys::keys::RouteKey;
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    for _ in 0..10 {
        let key = RouteKey::create();

        // publish message `bar` on channel foo
        client.publish("people.new", key.into()).await?;
    }

    Ok(())
}
