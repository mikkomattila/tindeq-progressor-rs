#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use std::error::Error;

mod app;
mod progressor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to tindeq-progressor-rs.");
    if let Err(err) = app::run().await {
        eprintln!("An error occurred: {}", err);
    }
    println!("\n\nApplication finished.");
    Ok(())
}
