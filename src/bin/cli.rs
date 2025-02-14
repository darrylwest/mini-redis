use mini_redis::{client, DEFAULT_PORT};

use bytes::Bytes;
use clap::{Parser, Subcommand};
use std::num::ParseIntError;
use std::str;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(name = "db-cli", version, author, about = "Issue Redis commands")]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(name = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[clap(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[derive(Subcommand, Debug)]
enum Command {
    Ping {
        /// Message to ping
        msg: Option<String>,
    },
    Count {
        count: Option<String>,
    },
    /// Get the value of key.
    Get {
        /// Name of key to get
        key: String,
    },
    /// Set key to hold the string value.
    Set {
        /// Name of key to set
        key: String,

        /// Value to set.
        value: String,

        /// expire from millis
        expires: Option<String>,
    },
    ///  Publisher to send a message to a specific channel.
    Publish {
        /// Name of channel
        channel: String,

        /// Message to publish
        message: String,
    },
    /// Subscribe a client to a specific channel or channels.
    Subscribe {
        /// Specific channel or channels
        channels: Vec<String>,
    },
}

/// Entry point for CLI tool.
///
/// The `[tokio::main]` annotation signals that the Tokio runtime should be
/// started when the function is called. The body of the function is executed
/// within the newly spawned runtime.
///
/// `flavor = "current_thread"` is used here to avoid spawning background
/// threads. The CLI tool use case benefits more by being lighter instead of
/// multi-threaded.
#[tokio::main(flavor = "current_thread")]
async fn main() -> mini_redis::Result<()> {
    // Enable logging
    tracing_subscriber::fmt::try_init()?;

    // Parse command line arguments
    let cli = Cli::parse();

    // Get the remote address to connect to
    let addr = format!("{}:{}", cli.host, cli.port);

    // Establish a connection
    let mut client = client::connect(&addr).await?;

    // Process the requested command
    match cli.command {
        Command::Ping { msg } => {
            let value = client.ping(msg).await?;
            if let Ok(string) = str::from_utf8(&value) {
                println!("\"{}\"", string);
            } else {
                println!("{:?}", value);
            }
        }
        Command::Count { .. } => {
            let value = client.count().await?;
            println!("count: {}", value.count());
        }
        Command::Get { key } => {
            if let Some(value) = client.get(&key).await? {
                if let Ok(string) = str::from_utf8(&value) {
                    println!("\"{}\"", string);
                } else {
                    println!("{:?}", value);
                }
            } else {
                println!("(nil)");
            }
        }
        Command::Set {
            key,
            value,
            expires: None,
        } => {
            client.set(&key, bytes_from_str(&value)).await?;
            println!("OK");
        }
        Command::Set {
            key,
            value,
            expires: Some(expires),
        } => {
            let exp = duration_from_ms_str(&expires).expect("should parse to duration");
            client
                .set_expires(&key, bytes_from_str(&value), exp)
                .await?;
            println!("OK");
        }
        Command::Publish { channel, message } => {
            client.publish(&channel, bytes_from_str(&message)).await?;
            println!("Publish OK");
        }
        Command::Subscribe { channels } => {
            if channels.is_empty() {
                return Err("channel(s) must be provided".into());
            }
            let mut subscriber = client.subscribe(channels).await?;

            // await messages on channels
            while let Some(msg) = subscriber.next_message().await? {
                println!(
                    "got message from the channel: {}; message = {:?}",
                    msg.channel, msg.content
                );
            }
        }
    }

    Ok(())
}

fn duration_from_ms_str(src: &str) -> Result<Duration, ParseIntError> {
    let ms = src.parse::<u64>()?;
    Ok(Duration::from_millis(ms))
}

fn bytes_from_str(src: &str) -> Bytes {
    Bytes::from(src.to_string())
}
