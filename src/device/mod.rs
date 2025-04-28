use reqwest::{Client, ClientBuilder};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Model {
    MitraLC,
    AskeyLC,
    MitraEconet,
    AskeyEconet,
    MitraWiFi6,
    AskeyWiFi6,
}

impl Model {
    pub(crate) fn from_sap_code(sap: &str) -> Option<Self> {
        match sap {
            "0192-0431-0" | "0192-0432-1" => Some(Model::MitraLC),
            "0192-0429-8" | "0192-0430-9" | "0192-0438-7" | "0192-0446-6" => Some(Model::AskeyLC),
            "0192-0452-2" | "0192-0453-3" | "0192-0476-0" | "0192-0477-0" => {
                Some(Model::MitraEconet)
            }
            "0192-0450-0" | "0192-0458-8" | "0192-0475-0" => Some(Model::AskeyEconet),
            "0192-0483-0" => Some(Model::MitraWiFi6),
            "0192-0484-0" => Some(Model::AskeyEconet),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    pub(crate) ip_address: String,
    pub(crate) mac_address: String,
    pub(crate) serial_number: String,
    pub(crate) admin_password: String,
    pub(crate) sap_code: String,
    pub(crate) model: Model,
    pub(crate) gpon_sn: String,
    pub(crate) firmware_version: String,
    pub index_data: IndexData,
    pub meta_data: MetaData,
}

impl Device {
    pub fn new(
        ip_addr: &str,
        mac_address: &str,
        serial_number: &str,
        admin_password: &str,
        sap_code: &str,
        gpon_sn: &str,
        firmware_version: &str,
    ) -> (Self, Client) {
        let client = Self::connect();
        let index_data = IndexData::default();
        let meta_data = MetaData::default();
        let ip_address = String::from(ip_addr);
        let mac_address = String::from(mac_address);
        let serial_number = String::from(serial_number);
        let admin_password = String::from(admin_password);
        let model = Model::from_sap_code(sap_code).unwrap();
        let sap_code = String::from(sap_code);
        let gpon_sn = String::from(gpon_sn);
        let firmware_version = String::from(firmware_version);

        (
            Device {
                ip_address,
                mac_address,
                serial_number,
                admin_password,
                sap_code,
                model,
                gpon_sn,
                firmware_version,
                index_data,
                meta_data,
            },
            client,
        )
    }

    fn connect() -> Client {
        let client = ClientBuilder::new().cookie_store(true).build().unwrap();

        client
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IndexData {
    pub gpon_status: String,
    pub optical_power: String,
    pub ppp_status: String,
    pub ppp_ipv4_gateway: String,
    pub wl_is_enabled_main_0: String,
    pub wl_ssid_main_0: String,
    pub wl_is_enabled_main_1: String,
    pub wl_ssid_main_1: String,
    pub ethernet_status: String,
    pub hpna_status: String,
}

impl IndexData {
    fn set_field(&mut self, var: &str, value: &str) {
        match var {
            "gponUp" => self.gpon_status = value.to_string(),
            "opticalPower" => self.optical_power = value.to_string(),
            "pppStatus" => self.ppp_status = value.to_string(),
            "pppIpv4Gateway" => self.ppp_ipv4_gateway = value.to_string(),
            "enetStatus" => self.ethernet_status = value.to_string(),
            "wlEnbl_main0" => self.wl_is_enabled_main_0 = value.to_string(),
            "wlSsid_main0" => self.wl_ssid_main_0 = value.to_string(),
            "wlEnbl_main1" => self.wl_is_enabled_main_1 = value.to_string(),
            "wlSsid_main1" => self.wl_ssid_main_1 = value.to_string(),
            "hpnaSelfIntfCfg" => self.hpna_status = value.to_string(),
            _ => (),
        };
    }

    pub fn from_hashmap(hashmap: HashMap<String, String>) -> Self {
        let mut index_data = IndexData::default();

        for (key, value) in hashmap {
            index_data.set_field(&key, &value);
        }

        index_data
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MetaData {
    pub(crate) mac_address: String,
    pub(crate) serial_number: String,
    pub(crate) model: String,
    pub(crate) gpon_sn: String,
    pub(crate) firmware_version: String,
}

impl MetaData {
    fn set_field(&mut self, var: &str, value: &str) {
        match var {
            "Modelo" => self.model = value.to_string(),
            "Versão do Software" => self.firmware_version = value.to_string(),
            "Número de Série" => self.serial_number = value.to_string(),
            "Número de Série GPON" => self.gpon_sn = value.to_string(),
            "Endereço MAC da WAN" => self.mac_address = value.to_string(),
            _ => (),
        }
    }

    pub fn from_hashmap(hashmap: HashMap<String, String>) -> Self {
        let mut meta_data = MetaData::default();

        for (key, value) in hashmap {
            meta_data.set_field(&key, &value);
        }

        meta_data
    }

    pub fn sap(&self) -> Option<&str> {
        match self.model.as_str() {
            "RTF3505VW-N1" => Some("0192-0429-8"),
            "RTF3505VW-N2" => Some("0192-0430-9"),
            "GPT-2541GNAC-N1" => Some("0192-0431-0"),
            "GPT-2541GNAC-GV" | "GPT-2541GNAC-N2" | "GPT-2541GN2A-N2" => Some("0192-0432-1"),
            "RTF3507VW-N1" => Some("0192-0438-7"),
            "GPT-2731GN2A4P-N1" => Some("0192-0442-2"),
            "RTF3507VW-N2" => Some("0192-0446-6"),
            "GPT-2731GN2A4P-N2" => Some("0192-0449-9"),
            "RTF8115VW" => Some("0192-0450-0"),
            "GPT-2741GNAC-N1" => Some("0192-0452-2"),
            "GPT-2741GNAC-N2" => Some("0192-0453-3"),
            "RTF8117VW" => Some("0192-0458-8"),
            "RTF8115VW-SV" => Some("0192-0475-0"),
            "GPT-2741GNAC-N1-SV" => Some("0192-0476-0"),
            "GPT-2741GNAC-N2-SV" => Some("0192-0477-0"),
            "GPT-2742GX4X5" => Some("0192-0483-0"),
            "RTF8225VW" => Some("0192-0484-0"),
            _ => None,
        }
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
