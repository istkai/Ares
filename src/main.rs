pub mod crypt;
pub mod device;
pub mod fetch;
pub mod test;

use crate::device::Device;
use crate::test::*;

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
    //     "0192-0450-0",
    //     "TLCM00BA1D59",
    //     "BR_SV_g13.12_RTF_TEF001_V8.30_V020",
    // );

    let (mut device_askey_econet, client_askey_econet) = Device::new(
        "192.168.15.1",
        "F454209AF2E1",
        "F454209AF2E1",
        "65rffsmj",
        "0192-0458-8",
        "TLCM00FC32AC",
        "BR_SV_g13.12_RTF_TEF001_V8.30_V020",
    );

    // let (mut device_askey_econet, client_askey_econet) = Device::new(
    //     "192.168.15.1",
    //     "900A6241A451",
    //     "900A6241A451",
    //     "hqq5v95y",
    //     "0192-0475-0",
    //     "INVP70653563",
    //     "BR_SG_g13.12_RTF_TEF001_V8.30_V020",
    // );

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
        .await
        .map(|device| -> Result<Device, ()> {
            if device.index_data.ppp_status.is_empty() {
                println!("[90]: Incorrect password. Please verify");
                std::process::exit(0);
            } else {
                Ok(device)
            }
        })
        .expect("")
        .unwrap()
        .fetch_meta_data(&client_askey_econet)
        .await?;

    for status in assert_meta_data(&device_askey_econet).iter() {
        println!("[{}]: {}", status.0, status.1);
    }

    for status in assert_index_data(&device_askey_econet).iter() {
        println!("[{}]: {}", status.0, status.1);
    }

    Ok(())
}
