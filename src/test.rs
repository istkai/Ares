use crate::device::Device;
use std::error::Error;

pub fn assert_meta_data(device: &Device) -> Vec<i32> {
    let mut status = Vec::new();

    if device.mac_address != device.meta_data.mac_address {}

    status
}
