use std::{error::Error};
use Reseaux::device::Device;
use Reseaux::log::Log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let (device_tg, client_tg) = Device::new(
        "192.168.15.1",
        "C03DD93B97E0",
        "C03DD93B97E0",
        "d1f87fa2",
        "Mitra-Econet",
        "MSTC393A9372",
        "BR_g8.7_1.11(WVK.0)b45"
    );
    
    // let (device_home, client_home) = Device::new(
    //     "192.168.15.1",
    //     "78E9CF070231",
    //     "78E9CF070231",
    //     "j69qjm4z",
    //     "Askey-Econet",
    //     "TLCM00BA1D59",
    //     "BR_SV_g13.12_RTF_TEF001_V8.30_V020"
    // );

    // let index_data_home = device_home.login_to_index(&client_home)
    //     .await?
    //     .fetch_index_data(&client_home)
    //     .await?;
    //
    // let meta_data_home = device_home.fetch_meta_data(&client_home).await?;
    //
    // println!("{:?}\n{:?}\n{:?}", device_home, index_data_home, meta_data_home);
    //
    // let log_home = Log::from_device(&device_home, &index_data_home)?;

    let index_data_tg = device_tg.login_to_index(&client_tg)
        .await?
        .fetch_index_data(&client_tg)
        .await?;

    let meta_data_tg = device_tg.fetch_meta_data(&client_tg).await?;

    println!("{:?}\n{:?}\n{:?}", device_tg, client_tg, index_data_tg);

    let log_tg = Log::from_device(&device_tg, &index_data_tg, &meta_data_tg)?;

    Ok(())
}