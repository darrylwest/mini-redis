//! Subscribe to a redis channel example.
//!
//! A simple client that connects to a mini-redis server, subscribes to "foo" and "bar" channels
//! and awaits messages published on those channels
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

#![warn(rust_2018_idioms)]

use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let client = client::connect("127.0.0.1:6379").await?;

    // subscribe to channel foo
    let mut subscriber = client.subscribe(vec!["people.new".into()]).await?;

    let mut list: Vec<String> = vec![];

    // await messages on channel foo
    loop {
        if let Some(msg) = subscriber.next_message().await? {
            let vkey = msg.content.to_vec();
            let skey = &String::from_utf8(vkey).unwrap();

            list.push(skey.to_string());

            assert_eq!(skey.len(), 16);

            println!(
                "got message from the channel: {}; message = {}, len: {}",
                msg.channel, skey, list.len(),
            );

        }
    }

}
