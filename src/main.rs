use reseaux::{device::Device, test::*};
use std::error::Error;

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

    // let (mut device_askey_econet, client_askey_econet) = Device::new(
    //     "192.168.15.1",
    //     "78E9CF070231",
    //     "78E9CF070231",
    //     "j69qjm4z",
    //     AskeyEconet,
    //     "TLCM00BA1D59",
    //     "BR_SV_g13.12_RTF_TEF001_V8.30_V020",
    // );

    let (mut device_askey_econet, client_askey_econet) = Device::new(
        "192.168.15.1",
        "900A6241A451",
        "900A6241A451",
        "hqq5v95y",
        "0192-0475-0",
        "INVP70653563",
        "BR_SG_g13.12_RTF_TEF001_V8.30_V020",
    );

    // let (mut device_askey_lc, client_askey_lc) = Device::new(
    //     "192.168.15.1",
    //     "94EAEAC2EC3F",
    //     "94EAEAC2EC3F",
    //     "e9et2d6g",
    //     Model::AskeyLC,
    //     "TLCM00246371",
    //     "BR_SV_g000_R3505VWN1001_s42",
    // );

    device_askey_econet = device_askey_econet
        .login_to_index(&client_askey_econet)
        .await?
        .fetch_index_data(&client_askey_econet)
        .await?
        .fetch_meta_data(&client_askey_econet)
        .await?;

    // tokio::time::sleep(time::Duration::from_secs(5)).await;

    dbg!(&device_askey_econet);
    dbg!(assert_meta_data(&device_askey_econet));
    dbg!(assert_index_data(&device_askey_econet));

    Ok(())
}
