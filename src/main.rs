use std::{error::Error};
use std::sync::{Arc, Mutex};
use Reseaux::device::Device;
use Reseaux::log::Log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let device = Device::new(
        "192.168.15.1",
        "C03DD93B97E0",
        "C03DD93B97E0",
        "d1f87fa2",
        "Mitra-Econet",
        "MSTC393A9372",
        "BR_g8.7_1.11(WVK.0)b45"
    );

    dbg!(device.clone().login_to_index()
        .await?
        .fetch_index_data()
        .await?);

    let mut log = Log::from_device(device)?;

    Ok(())
}