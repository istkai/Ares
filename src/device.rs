use std::collections::HashMap;
use std::error::Error;
use reqwest::{Client, ClientBuilder};

#[derive(Copy, Debug, Clone)]
pub struct Device<'a> {
    pub (crate) ip_addr: &'a str,
    mac_address: &'a str,
    pub (crate) serial_number: &'a str,
    pub (crate) admin_password: &'a str,
    pub (crate) model: &'a str,
    gpon_sn: &'a str,
    firmware_version: &'a str,
}

impl<'a> Device<'a> {

    pub fn new(
        ip_addr: &'a str,
        mac_address: &'a str,
        serial_number: &'a str,
        admin_password: &'a str,
        model: &'a str,
        gpon_sn: &'a str,
        firmware_version: &'a str) -> (Self, Client) {

        let client = Self::connect();

        (Device {
            ip_addr,
            mac_address,
            serial_number,
            admin_password,
            model,
            gpon_sn,
            firmware_version,
        },
            client,
        )

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
    fn new(gpon_status: &str,
               optical_power: &str,
               ppp_status: &str,
               ppp_ipv4_gateway: &str,
               wl_is_enabled_main_0: &str,
               wl_ssid_main_0: &str,
               wl_is_enabled_main_1: &str,
               wl_ssid_main_1: &str,
               ethernet_status: &str) -> Self {

        let gpon_status = String::from(gpon_status);
        let optical_power = String::from(optical_power);
        let ppp_status = String::from(ppp_status);
        let ppp_ipv4_gateway = String::from(ppp_ipv4_gateway);
        let wl_is_enabled_main_0 = String::from(wl_is_enabled_main_0);
        let wl_ssid_main_0 = String::from(wl_ssid_main_0);
        let wl_is_enabled_main_1 = String::from(wl_is_enabled_main_1);
        let wl_ssid_main_1 = String::from(wl_ssid_main_1);
        let ethernet_status = String::from(ethernet_status);

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
            gpon_status: String::new(),
            optical_power: String::new(),
            ppp_status: String::new(),
            ppp_ipv4_gateway: String::new(),
            wl_is_enabled_main_0: String::new(),
            wl_ssid_main_0: String::new(),
            wl_is_enabled_main_1: String::new(),
            wl_ssid_main_1: String::new(),
            ethernet_status: String::new()
        }
    }

}

#[derive(Debug)]
pub struct MetaData {
    mac_address: String,
    serial_number: String,
    model: String,
    gpon_sn: String,
    firmware_version: String
}

impl MetaData {
    fn new(mac_address: &str,
           serial_number: &str,
           model: &str,
           gpon_sn: &str,
           firmware_version: &str) -> Self {

        let mac_address = String::from(mac_address);
        let serial_number = String::from(serial_number);
        let model = String::from(model);
        let gpon_sn = String::from(gpon_sn);
        let firmware_version = String::from(firmware_version);

        MetaData {
            mac_address,
            serial_number,
            model,
            gpon_sn,
            firmware_version
        }

    }

    fn set_field(&mut self, var: &str, value: &str) -> Result<(), Box<dyn Error>> {
        match var {
            "Modelo" => Ok(self.model = value.to_string()),
            "Versão do Software" => Ok(self.firmware_version = value.to_string()),
            "Número de Série" => Ok(self.serial_number = value.to_string()),
            "Número de Série GPON" => Ok(self.gpon_sn = value.to_string()),
            "Endereço MAC da WAN" => Ok(self.mac_address = value.to_string()),
            _ => Ok(())
        }
    }

    pub fn from_hashmap(hashmap: HashMap<String, String>) -> Self {
        let mut meta_data = MetaData::default();

        for (key, value) in hashmap {
            meta_data.set_field(&key, &value).unwrap();
        }

        meta_data

    }
}

impl Default for MetaData {

    fn default() -> Self {
        MetaData {
            mac_address: String::new(),
            serial_number: String::new(),
            model: String::new(),
            gpon_sn: String::new(),
            firmware_version: String::new()
        }
    }
}