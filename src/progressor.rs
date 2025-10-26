use btleplug::api::{Central, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;

const SCAN_DURATION: Duration = Duration::from_secs(10);
const NAME: &str = "Progressor";
// pub const SERVICE_UUID: &str = "7e4e1701-1ea6-40c9-9dcc-13d34ffead57";
// pub const DATA_UUID: &str = "7e4e1702-1ea6-40c9-9dcc-13d34ffead57";
// pub const CONTROL_POINT_UUID: &str = "7e4e1703-1ea6-40c9-9dcc-13d34ffead57";

/// Represents a Tindeq Progressor device.
pub struct Progressor {
    peripheral: Peripheral,
}

impl Progressor {
    /**
     * Handles connection and interaction with a Tindeq Progressor device.
     * # Returns
     * * * `Ok(())` - If the connection and interaction are successful.
     * * `Err` - If an error occurs during connection or interaction.
     */
    pub async fn connect(&self) -> Result<(), Box<dyn Error>> {
        let properties = self
            .peripheral
            .properties()
            .await?
            .ok_or("Could not retrieve progressor properties.")?;

        let name = properties
            .local_name
            .ok_or("Progressor name was not found.")?;

        println!("\nConnecting to {}.", &name);
        self.peripheral.connect().await?;

        let is_connected = self.peripheral.is_connected().await?;
        if !is_connected {
            return Err("Failed to connect.".into());
        }

        // TODO: Do something useful with the Progressor device here

        Ok(())
    }

    /**
     * Disconnects from the Tindeq Progressor device.
     * # Returns
     * * * `Ok(())` - If the disconnection is successful.
     * * `Err` - If an error occurs during disconnection.
     */
    pub async fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        self.peripheral.disconnect().await?;
        Ok(())
    }
}

/**
 * Scans for a Tindeq Progressor device using the provided Bluetooth adapter.
 * # Arguments
 * * `adapter` - A reference to the Bluetooth adapter to use for scanning.
 * # Returns
 * * `Ok(Some(Progressor))` - If a Tindeq Progressor device is found.
 * * `Ok(None)` - If no Tindeq Progressor device is found.
 * * `Err` - If an error occurs during scanning.
 */
pub async fn find(adapter: &Adapter) -> Result<Option<Progressor>, Box<dyn Error>> {
    println!(
        "Scanning for devices for {} seconds...",
        SCAN_DURATION.as_secs()
    );
    adapter.start_scan(ScanFilter::default()).await?;
    time::sleep(SCAN_DURATION).await;

    for peripheral in adapter.peripherals().await? {
        if let Some(properties) = peripheral.properties().await? {
            if let Some(name) = properties.local_name {
                if name.contains(NAME) {
                    return Ok(Some(Progressor { peripheral }));
                }
            }
        }
    }

    Ok(None)
}
