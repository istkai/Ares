use crate::device::Device;

pub fn assert_meta_data(device: &Device) -> Vec<i32> {
    let mut status: Vec<i32> = Vec::with_capacity(5);

    if device.mac_address != device.meta_data.mac_address {
        status.push(-1);
    }

    if device.serial_number != device.meta_data.serial_number {
        status.push(-1);
    }

    if device.meta_data.sap().unwrap() != device.sap_code {
        status.push(-1);
    }

    if device.gpon_sn != device.meta_data.gpon_sn {
        status.push(-1);
    }

    if device.firmware_version != device.meta_data.firmware_version {
        status.push(-1);
    }

    if status.is_empty() {
        status.push(0);
    }

    status
}

pub fn assert_index_data(device: &Device) -> Vec<i32> {
    let mut temp: Vec<i32> = Vec::with_capacity(9);
    let mut status: Vec<i32> = Vec::with_capacity(9);

    todo!()
}
