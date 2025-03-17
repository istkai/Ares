use std::{error::Error};
use std::sync::{Arc, Mutex};
use Reseaux::device::Device;
use Reseaux::log::Log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let device = Device::new(
        "192.168.15.1",
        "78E9CF070231",
        "78E9CF070231",
        "j69qjm4z",
        "RTF8115VW",
        "TLCM00BA1D59",
        "BR_SV_g13.12_RTF_TEF001_V8.30_V020"
    );
    
    let mut log = Log::from_device(&device)?;

    dbg!(device.login_to_index()
        .await?
        .fetch_index_data()
        .await?);
    
    Ok(())
}
