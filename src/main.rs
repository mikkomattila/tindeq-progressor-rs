#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use btleplug::api::{Central, Manager as _, Peripheral};
use btleplug::platform::Manager;
use std::error::Error;

mod progressor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = match adapters.into_iter().next() {
        Some(adapter) => adapter,
        None => {
            eprintln!("No Bluetooth adapters found.");
            return Ok(());
        }
    };

    println!("Using {}.", adapter.adapter_info().await?);

    if let Some(progressor_peripheral) = progressor::find(&adapter).await? {
        if let Err(err) = progressor::connect(&progressor_peripheral).await {
            eprintln!("Error establishing connection: {}.", err);
        } else {
            println!("Connection established. Press Ctrl+C to disconnect and exit.");
            tokio::signal::ctrl_c().await?;
            progressor_peripheral.disconnect().await?;
            println!("\nDisconnected.");
        }
    } else {
        eprintln!("Tindeq Progressor not found.");
    }

    Ok(())
}
