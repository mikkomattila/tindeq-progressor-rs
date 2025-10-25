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
        eprintln!("No Bluetooth adapters found");
        return Ok(());
    }

    for adapter in adapters.into_iter() {
        println!("Starting scan on {}", adapter.adapter_info().await?);

        adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(SCAN_DURATION).await;

        let peripherals = adapter.peripherals().await?;
        if peripherals.is_empty() {
            eprintln!("No bluetooth peripherals found");
            continue;
        }

        let mut progressor_found = false;
        for peripheral in peripherals.iter() {
            if let Some(properties) = peripheral.properties().await? {
                if let Some(name) = properties.local_name {
                    if name.contains(progressor::NAME) {
                        if let Err(err) = progressor::handle(peripheral).await {
                            eprintln!("Error handling progressor {}: {}", name, err);
                        }
                        progressor_found = true;
                        break;
                    }
                }
            }
        }

        if !progressor_found {
            eprintln!("No Progressor found");
        }
    }
    Ok(())
}
