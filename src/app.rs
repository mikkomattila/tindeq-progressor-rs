use btleplug::api::{Central, Manager as _};
use btleplug::platform::{Adapter, Manager};
use std::error::Error;

use crate::progressor;

/**
Runs the main application logic to find and connect to a Tindeq Progressor device.
# Returns
 * `Ok(())` - If the application runs successfully.
 * `Err` - If an error occurs during execution.
*/
pub async fn run() -> Result<(), Box<dyn Error>> {
    let Some(adapter) = get_adapter().await? else {
        return Err("No Bluetooth adapters found.".into());
    };

    let Some(progressor) = progressor::find(&adapter).await? else {
        return Err("Tindeq Progressor not found.".into());
    };

    let progressor_name = progressor.name().to_string();
    if let Err(err) = progressor.connect().await {
        eprintln!(
            "Error establishing connection to {}: {}.",
            progressor_name, err
        );
        return Err("Error establishing connection to Tindeq Progressor.".into());
    }

    println!(
        "Connection established to {}. Press Ctrl+C to disconnect and exit.",
        progressor_name
    );
    tokio::signal::ctrl_c().await?;
    progressor.disconnect().await?;

    Ok(())
}

/**
Retrieves the first available Bluetooth adapter.
# Returns
 * `Ok(Some(Adapter))` - If a Bluetooth adapter is found.
 * `Ok(None)` - If no Bluetooth adapters are found.
 * `Err` - If an error occurs while retrieving adapters.
*/
async fn get_adapter() -> Result<Option<Adapter>, Box<dyn Error>> {
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
