#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::Manager;
use std::error::Error;
use std::time::Duration;
use tokio::time;

mod progressor;

const SCAN_DURATION: Duration = Duration::from_secs(10);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    if adapters.is_empty() {
        eprintln!("No Bluetooth adapters found, exiting...");
        return Ok(());
    }

    for adapter in adapters.into_iter() {
        println!(
            "Scanning for devices using {}.",
            adapter.adapter_info().await?
        );

        adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(SCAN_DURATION).await;

        let peripherals = adapter.peripherals().await?;
        if peripherals.is_empty() {
            eprintln!("No bluetooth peripherals found, exiting...");
            return Ok(());
        }

        let mut progressor_found = false;
        for peripheral in peripherals.iter() {
            if let Some(properties) = peripheral.properties().await? {
                if let Some(name) = properties.local_name {
                    if name.contains(progressor::NAME) {
                        if let Err(err) = progressor::connect(peripheral).await {
                            eprintln!("Error connecting to {}: {}.", name, err);
                        } else {
                            println!(
                                "Connection established. Press Ctrl+C to disconnect and exit."
                            );

                            tokio::signal::ctrl_c().await?;

                            peripheral.disconnect().await?;
                            println!("\nDisconnected, exiting...");
                        }
                        progressor_found = true;
                        break;
                    }
                }
            }
        }

        if !progressor_found {
            eprintln!("Progressor not found, exiting...");
        }
    }
    Ok(())
}
