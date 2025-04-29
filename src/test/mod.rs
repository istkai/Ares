use crate::device::{Device, Model};

mod askey_econet;
mod askey_lc;
mod mitra_econet;
mod mitra_lc;
mod mitra_wifi6;

pub fn meta_test(device: &Device) -> Vec<(i32, &str)> {
    match device.model {
        Model::MitraLC => mitra_lc::meta_test(device),
        Model::AskeyLC => askey_lc::meta_test(device),
        Model::MitraEconet => mitra_econet::meta_test(device),
        Model::AskeyEconet | Model::AskeyWiFi6 => askey_econet::meta_test(device),
        Model::MitraWiFi6 => mitra_wifi6::meta_test(device),
    }
}

pub fn index_test(device: &Device) -> Vec<(i32, &str)> {
    match device.model {
        Model::MitraLC => mitra_lc::index_test(device),
        Model::AskeyLC => askey_lc::index_test(device),
        Model::MitraEconet => mitra_econet::index_test(device),
        Model::AskeyEconet | Model::AskeyWiFi6 => askey_econet::index_test(device),
        Model::MitraWiFi6 => mitra_wifi6::index_test(device),
    }
}
