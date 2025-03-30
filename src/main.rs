use reseaux::device::{Device, Model::*};
use std::error::Error;
use std::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let (device_mitra_econet, client_mitra_econet) = Device::new(
    //     "192.168.15.1",
    //     "C03DD93B97E0",
    //     "C03DD93B97E0",
    //     "d1f87fa2",
    //     "Mitra-Econet",
    //     "MSTC393A9372",
    //     "BR_g8.7_1.11(WVK.0)b45"
    // );

    let (mut device_askey_econet, client_askey_econet) = Device::new(
        "192.168.15.1",
        "78E9CF070231",
        "78E9CF070231",
        "j69qjm4z",
        AskeyEconet,
        "TLCM00BA1D59",
        "BR_SV_g13.12_RTF_TEF001_V8.30_V020",
    );

    // let (device_askey_lc, client_askey_lc) = Device::new(
    //     "192.168.15.1",
    //     "94EAEAC2EC3F",
    //     "94EAEAC2EC3F",
    //     "e9et2d6g",
    //     "Askey-LC",
    //     "TLCM00246371",
    //     "BR_SV_g000_R3505VWN1001_s42"
    // );

    device_askey_econet = device_askey_econet
        .login_to_index(&client_askey_econet)
        .await?
        .fetch_index_data(&client_askey_econet)
        .await?
        .fetch_meta_data(&client_askey_econet)
        .await?;

    tokio::time::sleep(time::Duration::from_secs(5)).await;

    dbg!(&device_askey_econet);

    let _log_device_askey_econet = &device_askey_econet.log.write()?;

    // let meta_data_askey_econet = device_askey_econet
    //     .fetch_meta_data(&client_askey_econet)
    //     .await?;

    // let log_device_askey_econet = Log::from_device(&device_askey_econet, &index_data_askey_econet, &meta_data_askey_econet)?;
    //
    // let index_data_mitra_econet = device_mitra_econet.login_to_index(&client_mitra_econet)
    //     .await?
    //     .fetch_index_data(&client_mitra_econet)
    //     .await?;
    //
    // let meta_data_mitra_econet = device_mitra_econet.fetch_meta_data(&client_mitra_econet).await?;

    // println!("{:?}\n{:?}\n{:?}", device_mitra_econet, client_mitra_econet, index_data_mitra_econet);
    //
    // let log_device_mitra_econet = Log::from_device(&device_mitra_econet, &index_data_mitra_econet, &meta_data_mitra_econet)?;

    // let index_data_askey_lc = device_askey_lc.login_to_index(&client_askey_lc)
    //     .await?
    //     .fetch_index_data(&client_askey_lc)
    //     .await?;
    //
    // let meta_data_askey_lc = device_askey_lc.fetch_meta_data(&client_askey_lc).await?;
    //
    // println!("{:?}\n{:?}\n{:?}", device_askey_lc, index_data_askey_lc, meta_data_askey_lc);

    // let log_device_askey_lc = Log::from_device(&device_askey_lc, &index_data_askey_lc, &meta_data_askey_lc)?;

    Ok(())
}
