use crate::device::Device;

pub fn assert_meta_data(device: &Device) -> Vec<(i32, &str)> {
    let mut status: Vec<(i32, &str)> = Vec::with_capacity(5);

    if device.mac_address != device.meta_data.mac_address {
        status.push((-1, "Incorrect MAC Adress"));
    }

    if device.serial_number != device.meta_data.serial_number {
        status.push((-1, "Incorrect Serial Number"));
    }

    if device.meta_data.sap().unwrap_or_default() != device.sap_code {
        status.push((-1, "Incorrect Device Model"));
    }

    if device.firmware_version != device.meta_data.firmware_version {
        status.push((-1, "Firmware out of date"));
    }

    status
}

pub fn assert_index_data(device: &Device) -> Vec<(i32, &str)> {
    let mut status = Vec::with_capacity(9);

    if device.index_data.gpon_status != "1" {
        status.push((70, "Optical connection not established"));
    }

    let mut optical_power = device.index_data.optical_power.split(";").take(2);

    let tx = optical_power
        .next()
        .unwrap_or_default()
        .split(':')
        .nth(1)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or("-40.0")
        .parse::<f32>()
        .unwrap_or_default();

    let rx = optical_power
        .next()
        .unwrap_or_default()
        .split(':')
        .nth(1)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or("-40.0")
        .parse::<f32>()
        .unwrap_or_default();

    dbg!(&tx, &rx);

    if rx > -16.0 {
        status.push((70, "Optical Power too high"));
    }

    if rx <= -28.0 {
        status.push((70, "Optical Power too low"));
    }

    if device.index_data.wl_is_enabled_main_0 != "1" {
        status.push((60, "WLAN 2.4 GHz not functional"));
    }

    if device.index_data.wl_is_enabled_main_1 != "1" {
        status.push((60, "WLAN 5 GHz not functional"));
    }

    let mut port: u8 = 1;

    for i in [9, 20, 31, 42] {
        if device
            .index_data
            .ethernet_status
            .char_indices()
            .nth(i)
            .unwrap_or_default()
            .1
            == '0'
        {
            status.push((
                72,
                format!("Failed to communicate with port {}.", port).leak(),
            ));
        }
        port += 1;
    }

    status
}
