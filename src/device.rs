use std::collections::HashMap;
use std::error::Error;
use std::{fs, fs::File};
use reqwest::{Client, ClientBuilder};

#[derive(Debug, Clone)]
pub struct Device {
    pub ip_addr: String,
    pub mac_addr: String,
    pub serial_number: String,
    pub admin_password: String,
    pub model: String,
    pub gpon_sn: String,
    pub firmware_version: String,
    pub client: Client,
    pub index_data: IndexData,
}

impl Device {

    pub fn new(
        ip_addr: &str,
        mac_addr: &str,
        serial_number: &str,
        admin_password: &str,
        model: &str,
        gpon_sn: &str,
        firmware_version: &str) -> Self {

        let ip_addr = ip_addr.to_string();
        let mac_addr = mac_addr.to_string();
        let serial_number = serial_number.to_string();
        let admin_password = admin_password.to_string();
        let model = model.to_string();
        let gpon_sn = gpon_sn.to_string();
        let firmware_version = firmware_version.to_string();
        let client = Self::connect();
        let index_data = IndexData::default();

        Device {
            ip_addr,
            mac_addr,
            serial_number,
            admin_password,
            model,
            gpon_sn,
            firmware_version,
            client,
            index_data,
        }

    }

    fn connect() -> Client {
        let client = ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap();

        client
    }

}

#[derive(Debug, Clone)]
pub struct IndexData {
    gpon_status: String,
    optical_power: String,
    ppp_status: String,
    ppp_ipv4_gateway: String,
    wl_is_enabled_main_0: String,
    wl_ssid_main_0: String,
    wl_is_enabled_main_1: String,
    wl_ssid_main_1: String,
    ethernet_status: String
}

impl IndexData {
    fn new(gpon_status: String,
           optical_power: String,
           ppp_status: String,
           ppp_ipv4_gateway: String,
           wl_is_enabled_main_0: String,
           wl_ssid_main_0: String,
           wl_is_enabled_main_1: String,
           wl_ssid_main_1: String,
           ethernet_status: String) -> Self {

        IndexData {
            gpon_status,
            optical_power,
            ppp_status,
            ppp_ipv4_gateway,
            wl_is_enabled_main_0,
            wl_ssid_main_0,
            wl_is_enabled_main_1,
            wl_ssid_main_1,
            ethernet_status
        }

    }

    fn set_field(&mut self, var: &str, value: &str) -> Result<(), Box<dyn Error>> {
        match var {
            "gponUp" => Ok(self.gpon_status = value.to_string()),
            "opticalPower" => Ok(self.optical_power = value.to_string()),
            "pppStatus" => Ok(self.ppp_status = value.to_string()),
            "pppIpv4Gateway" => Ok(self.ppp_ipv4_gateway = value.to_string()),
            "enetStatus" => Ok(self.ethernet_status = value.to_string()),
            "wlEnbl_main0" => Ok(self.wl_is_enabled_main_0 = value.to_string()),
            "wlSsid_main0" => Ok(self.wl_ssid_main_0 = value.to_string()),
            "wlEnbl_main1" => Ok(self.wl_is_enabled_main_1 = value.to_string()),
            "wlSsid_main1" => Ok(self.wl_ssid_main_1 = value.to_string()),
            _ => Err(format!("Unknown variable {}", var).into())
        }
    }

    pub fn from_hashmap(hashmap: HashMap<String, String>) -> Self {
        let mut index_data = IndexData::default();

        for (key, value) in hashmap {
            index_data.set_field(&key, &value).unwrap();
        }

        index_data

    }
}

impl Default for IndexData {

    fn default() -> Self {
        IndexData {
            gpon_status: "".to_string(),
            optical_power: "".to_string(),
            ppp_status: "".to_string(),
            ppp_ipv4_gateway: "".to_string(),
            wl_is_enabled_main_0: "".to_string(),
            wl_ssid_main_0: "".to_string(),
            wl_is_enabled_main_1: "".to_string(),
            wl_ssid_main_1: "".to_string(),
            ethernet_status: "".to_string()
        }
    }
}
