use btleplug::api::Peripheral as _;
use btleplug::platform::Peripheral;
use std::error::Error;

pub const NAME: &str = "Progressor";
// pub const SERVICE_UUID: &str = "7e4e1701-1ea6-40c9-9dcc-13d34ffead57";
// pub const DATA_UUID: &str = "7e4e1702-1ea6-40c9-9dcc-13d34ffead57";
// pub const CONTROL_POINT_UUID: &str = "7e4e1703-1ea6-40c9-9dcc-13d34ffead57";

pub async fn handle(progressor: &Peripheral) -> Result<(), Box<dyn Error>> {
    let properties = progressor.properties().await?.unwrap_or_default();
    let name = properties
        .local_name
        .unwrap_or_else(|| String::from("(unknown)"));

    println!("Connecting to {}", &name);
    progressor.connect().await?;

    let is_connected = progressor.is_connected().await?;
    println!("Now connected to {}", &name);

    if is_connected {
        // println!("Discovering services for {}", &name);
        // peripheral.discover_services().await?;

        // for service in peripheral.services() {
        //     println!(
        //         "  Service UUID: {}, Primary: {}",
        //         service.uuid, service.primary
        //     );
        //     for characteristic in service.characteristics {
        //         println!("    Characteristic: {:?}", characteristic);
        //     }
        // }

        // TODO: Do something useful with the Progressor device here

        println!("Disconnecting from {}", &name);
        progressor.disconnect().await?;
    }
    Ok(())
}
