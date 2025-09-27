pub mod crypt;
pub mod device;
pub mod fetch;
pub mod test;
use std::io::{self, Write};

use crate::device::Device;
use crate::test::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sap_code = String::new();
    let mut serial_number = String::new();
    let mut mac_address = String::new();
    let mut admin_password = String::new();

    print!("Codigo SAP: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut sap_code).unwrap_or_default();

    print!("Serial: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut serial_number).unwrap_or_default();

    print!("MAC Address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mac_address).unwrap_or_default();

    print!("Senha: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut admin_password).unwrap_or_default();

    let (mut device, client) = Device::new(
        serial_number.trim(),
        mac_address.trim(),
        admin_password.trim(),
        sap_code.trim(),
    );

        device = device
        .login_to_index(&client)
        .await?
        .fetch_index_data(&client)
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
        .fetch_meta_data(&client)
        .await?;

    // dbg!(&device);

    for status in meta_test(&device).iter() {
        println!("[{}]: {}", status.0, status.1);
    }

    for status in index_test(&device).iter() {
        println!("[{}]: {}", status.0, status.1);
    }

    Ok(())
}
