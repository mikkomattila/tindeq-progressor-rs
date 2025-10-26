use btleplug::api::{Central, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager};
use std::error::Error;

use crate::progressor;

/**
 * Runs the main application logic to find and connect to a Tindeq Progressor device.
 * # Returns
 * * * `Ok(())` - If the application runs successfully.
 * * `Err` - If an error occurs during execution.
 */
pub async fn run() -> Result<(), Box<dyn Error>> {
    if let Some(adapter) = get_adapter().await? {
        if let Some(progressor_peripheral) = progressor::find(&adapter).await? {
            if let Err(err) = progressor::connect(&progressor_peripheral).await {
                eprintln!("Error establishing connection: {}.", err);
            } else {
                println!("Connection established. Press Ctrl+C to disconnect and exit.");
                tokio::signal::ctrl_c().await?;
                progressor_peripheral.disconnect().await?;
            }
        } else {
            eprintln!("Tindeq Progressor not found.");
        }
    }
    Ok(())
}

/**
 * Retrieves the first available Bluetooth adapter.
 * # Returns
 * * * `Ok(Some(Adapter))` - If a Bluetooth adapter is found.
 * * `Ok(None)` - If no Bluetooth adapters are found.
 * * `Err` - If an error occurs while retrieving adapters.
 */
pub async fn get_adapter() -> Result<Option<Adapter>, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = adapters.into_iter().next();

    if let Some(adapter) = &adapter {
        println!("\nUsing {}.", adapter.adapter_info().await?);
    } else {
        eprintln!("No Bluetooth adapters found.");
    }

    Ok(adapter)
}
