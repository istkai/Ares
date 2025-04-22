use crate::log;
use crate::log::Log;
use reqwest::{Client, ClientBuilder};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Model {
    MitraLC,
    AskeyLC,
    MitraEconet,
    AskeyEconet,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Device<'a> {
    pub(crate) ip_addr: &'a str,
    mac_address: &'a str,
    pub(crate) serial_number: &'a str,
    pub(crate) admin_password: &'a str,
    pub(crate) model: Model,
    gpon_sn: &'a str,
    firmware_version: &'a str,
    pub index_data: IndexData,
    pub meta_data: MetaData,
    pub log: Log,
}

impl<'a> Device<'a> {
    pub fn new(
        ip_addr: &'a str,
        mac_address: &'a str,
        serial_number: &'a str,
        admin_password: &'a str,
        model: Model,
        gpon_sn: &'a str,
        firmware_version: &'a str,
    ) -> (Self, Client) {
        let client = Self::connect();
        let index_data = IndexData::default();
        let meta_data = MetaData::default();

        let log = Log::new(
            format!("./log/{}/log/", serial_number),
            format!("./log/{}/log/{}_rsx_log", serial_number, serial_number),
            format!("./log/{}/log/{}_xmd_log", serial_number, serial_number),
        );

        (
            Device {
                ip_addr,
                mac_address,
                serial_number,
                admin_password,
                model,
                gpon_sn,
                firmware_version,
                index_data,
                meta_data,
                log,
            },
            client,
        )
    }

    fn connect() -> Client {
        let client = ClientBuilder::new().cookie_store(true).build().unwrap();

        client
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct IndexData {
    gpon_status: String,
    optical_power: String,
    ppp_status: String,
    ppp_ipv4_gateway: String,
    wl_is_enabled_main_0: String,
    wl_ssid_main_0: String,
    wl_is_enabled_main_1: String,
    wl_ssid_main_1: String,
    ethernet_status: String,
}

impl IndexData {
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
            _ => Ok(println!(
                "Found unknown variable \"{}\" while fetching Index Data",
                var
            )),
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

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct MetaData {
    mac_address: String,
    serial_number: String,
    model: String,
    gpon_sn: String,
    firmware_version: String,
}

impl MetaData {
    fn set_field(&mut self, var: &str, value: &str) -> Result<(), Box<dyn Error>> {
        match var {
            "Modelo" => Ok(self.model = value.to_string()),
            "Versão do Software" => Ok(self.firmware_version = value.to_string()),
            "Número de Série" => Ok(self.serial_number = value.to_string()),
            "Número de Série GPON" => Ok(self.gpon_sn = value.to_string()),
            "Endereço MAC da WAN" => Ok(self.mac_address = value.to_string()),
            _ => Ok(()),
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

impl Into<HashMap<&str, String>> for IndexData {
    fn into<'a>(self) -> HashMap<&'static str, String> {
        let mut hashmap: HashMap<&str, String> = HashMap::new();

        hashmap.insert("gpon_status", self.gpon_status);
        hashmap.insert("optical_power:", self.optical_power);
        hashmap.insert("ppp_status", self.ppp_status);
        hashmap.insert("ppp_ipv4_gateway", self.ppp_ipv4_gateway);
        hashmap.insert("wl_is_enabled_main_0", self.wl_is_enabled_main_0);
        hashmap.insert("wl_ssid_main_0", self.wl_ssid_main_0);
        hashmap.insert("wl_is_enabled_main_1", self.wl_is_enabled_main_1);
        hashmap.insert("wl_ssid_main_1", self.wl_ssid_main_1);
        hashmap.insert("ethernet_status", self.ethernet_status);

        hashmap
    }
}

impl Into<HashMap<&str, String>> for MetaData {
    fn into(self) -> HashMap<&'static str, String> {
        let mut hashmap: HashMap<&str, String> = HashMap::new();

        hashmap.insert("mac_address", self.mac_address);
        hashmap.insert("serial_number", self.serial_number);
        hashmap.insert("model", self.model);
        hashmap.insert("gpon_sn", self.gpon_sn);
        hashmap.insert("firmware_version", self.firmware_version);

        hashmap
    }
}

