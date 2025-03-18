use std::{collections::HashMap};
use std::error::Error;
use std::ops::Index;
use reqwest::{header, Client};
use regex::Regex;
use crate::{crypt, device::{IndexData, Device}};

impl Device<'_> {
    fn handle_login_input_mitra_lc(&self, login_username: &str) -> Result<(String, String), Box<dyn Error>> {
        todo!()
    }

    fn handle_login_input_askey_lc(&self, login_username: &str) -> Result<(String, String), Box<dyn Error>> {
        todo!()
    }

    /// XOR each character with 0x1F for encryption purposes (`REQUIRED FOR LOGIN`)
    fn handle_login_input_mitra_econet(&self, login_username: &str) -> Result<(String, String), Box<dyn Error>> {
        let login_username = crypt::generate_md5_hash(login_username.as_bytes());

        let login_password = crypt::generate_md5_hash(self.admin_password.as_ref());

        Ok(
            (login_username, login_password)
        )
    }

    fn handle_login_input_askey_econet(&self, login_username: &str) -> Result<(String, String), Box<dyn Error>> {
        let login_username = crypt::bitwise_xor(login_username)?;

        let login_password = crypt::bitwise_xor(self.admin_password)?;

        Ok(
            (login_username, login_password)
        )
    }

    fn handle_login_input(&self, login_username: &str) -> Result<(String, String), Box<dyn Error>> {
        match self.model {
            "Mitra-LC" => {
                self.handle_login_input_mitra_lc(login_username)
            },
            "Askey-LC" => {
                self.handle_login_input_askey_lc(login_username)
            }
            "Mitra-Econet" => {
                self.handle_login_input_mitra_econet(login_username)
            },
            "Askey-Econet" => {
                self.handle_login_input_askey_econet(login_username)
            }
            _ => {
                Err(format!("Unknown device model: {}", self.model).into())
            }
        }
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

    pub async fn login_to_index(self, client: &Client) -> Result<Self, Box<dyn Error>> {
        let index_login_get_uri = "/login.asp";
        let index_login_get_url = format!("http://{}{}", self.ip_addr, index_login_get_uri);

        // GET index login page to capture cookies
        let index_login_get_response = client
            .get(&index_login_get_url)
            .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
            .header(header::REFERER, &index_login_get_url)
            .send()
            .await?;

        // POST login form to log in and fetch index page
        let login_data = self.handle_login_input("admin")?;
        let login_form = self.generate_login_form(login_data);
        let index_login_post_uri = "/cgi-bin/te_acceso_router.cgi";
        let index_login_post_url = format!("http://{}{}", self.ip_addr, index_login_post_uri);

        let index_login_post_response = client
            .post(&index_login_post_url)
            .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0")
            .header(header::REFERER, &index_login_post_url)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&login_form)
            .send()
            .await?;

        Ok(self)
        
    }

    pub async fn fetch_index_data(&self, client: &Client) -> Result<IndexData, Box<dyn Error>> {
        let index_data_get_uri = "/index_cliente.asp";
        let index_data_get_url = format!("http://{}{}", self.ip_addr, index_data_get_uri);
        let index_data_get_response = self.collect_variables_from_response(
            client
                .get(&index_data_get_url)
                .send()
                .await?
                .text()
                .await?
                .as_str()
        );
        
        let index_data = IndexData::from_hashmap(index_data_get_response);
        
        Ok(index_data)
        
    }
}
