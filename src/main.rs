pub mod crypt;
pub mod device;
pub mod fetch;
pub mod test;

use crate::device::Device;
use crate::test::*;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let (mut device_mitra_wifi6, client_mitra_wifi6) = Device::new(
    //     "192.168.15.1",
    //     "840BBBB4D8F1",
    //     "840BBBB4D8F1",
    //     "ubPnVrRf",
    //     "0192-0483-0",
    //     "MSTCFFF3020D",
    //     "GL_g1.13_100XNT0b17_2",
    // );

    // let (mut device_mitra_econet, client_mitra_econet) = Device::new(
    //     "192.168.15.1",
    //     "C03DD93B97E0",
    //     "C03DD93B97E0",
    //     "d1f87fa2",
    //     "0192-0453-3",
    //     "MSTC393A9372",
    //     "BR_g8.7_1.11(WVK.0)b45",
    // );

    let (mut device_askey_econet, client_askey_econet) = Device::new(
        "192.168.15.1",
        "78E9CF070231",
        "78E9CF070231",
        "j69qjm4z",
        "0192-0450-0",
        "TLCM00BA1D59",
        "BR_SV_g13.12_RTF_TEF001_V8.30_V020",
    );

    // let (mut device_askey_econet, client_askey_econet) = Device::new(
    //     "192.168.15.1",
    //     "F454209AF2E1",
    //     "F454209AF2E1",
    //     "65rffsmj",
    //     "0192-0458-8",
    //     "TLCM00FC32AC",
    //     "BR_SV_g13.12_RTF_TEF001_V8.30_V020",
    // );

    // let (mut device_askey_econet, client_askey_econet) = Device::new(
    //     "192.168.15.1",
    //     "900A6241A451",
    //     "900A6241A451",
    //     "hqq5v95y",
    //     "0192-0475-0",
    //     "INVP70653563",
    //     "BR_SG_g13.12_RTF_TEF001_V8.30_V020",
    // );

    // let (mut device_askey_wifi6, client_askey_wifi6) = Device::new(
    //     "192.168.15.1",
    //     "44896D288381",
    //     "44896D288381",
    //     "64dE9wJz",
    //     "0192-0484-0",
    //     "TLCM01E319AB",
    //     "BR_SV_g1.4_RTF_TEF004_V2.9",
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
                println!("[90]: Device did not respond or incorrect password");
                std::process::exit(0);
            } else {
                Ok(device)
            }
        })
        .expect("")
        .unwrap()
        .fetch_meta_data(&client_askey_econet)
        .await?;

    dbg!(&device_askey_econet);

    for status in meta_test(&device_askey_econet).iter() {
        println!("[{}]: {}", status.0, status.1);
    }

    for status in index_test(&device_askey_econet).iter() {
        println!("[{}]: {}", status.0, status.1);
    }

    Ok(())
}
