use std::{collections::HashMap};
use std::error::Error;
use reqwest::header;
use regex::Regex;
use crate::device::{IndexData, Device};

impl Device {
    /// XOR each character with 0x1F for encryption purposes (`REQUIRED FOR LOGIN`)
    fn handle_login_input(&self, login_username: &str) -> (String, String) {
        let login_username = login_username.chars()
            .map(|char| ((char as u8) ^ 0x1F) as char)
            .collect();

        let login_password = self.admin_password.chars()
            .map(|char| ((char as u8) ^ 0x1F) as char)
            .collect();

        (login_username, login_password)
        
    }

    fn generate_login_form<'a>(&self, (login_username, login_password): (String, String)) -> HashMap<&'a str, String> {
        let mut login_form = HashMap::new();

            login_form.insert("loginUsername", login_username);
            login_form.insert("loginPassword", login_password);
            login_form.insert("curWebPage", "/index_cliente.asp".to_string());

        login_form
        
    }

    fn collect_variables_from_response(&self, response: &str) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        let fetch_pattern = Regex::new(
            r#"var\s+(gponUp|opticalPower|pppStatus|pppIpv4Gateway|enetStatus|wlEnbl_main0|wlSsid_main0|wlEnbl_main1|wlSsid_main1)\s*=\s*['"]?([^"']+)['"]?;"#)
            .unwrap();

        for capture in fetch_pattern.captures_iter(response) {
            let key = capture[1].to_string();
            let value = capture[2].to_string();
            vars.insert(key, value);
        }
        
        vars
        
    }

    pub async fn login_to_index(self) -> Result<Self, Box<dyn Error>> {
        let index_login_get_uri = "/login.asp";
        let index_login_get_url = format!("http://{}{}", self.ip_addr, index_login_get_uri);

        // GET index login page to capture cookies
        let index_login_get_response = self.client
            .get(&index_login_get_url)
            .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
            .header(header::REFERER, &index_login_get_url)
            .send()
            .await?;

        // POST login form to log in and fetch index page
        let login_data = self.handle_login_input("admin");
        let login_form = self.generate_login_form(login_data);
        let index_login_post_uri = "/cgi-bin/te_acceso_router.cgi";
        let index_login_post_url = format!("http://{}{}", self.ip_addr, index_login_post_uri);

        let index_login_post_response = self.client
            .post(&index_login_post_url)
            .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
            .header(header::REFERER, &index_login_post_url)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&login_form)
            .send()
            .await?;

        Ok(self)
        
    }

    pub async fn fetch_index_data(mut self) -> Result<Self, Box<dyn Error>> {
        let index_data_get_uri = "/index_cliente.asp";
        let index_data_get_url = format!("http://{}{}", self.ip_addr, index_data_get_uri);
        let index_data_get_response = self.collect_variables_from_response(
            self.client
            .get(&index_data_get_url)
            .send()
            .await?
            .text()
            .await?
            .as_str()
        );
        
        self.index_data = IndexData::from_hashmap(index_data_get_response);
        
        Ok(self)
        
    }
}
