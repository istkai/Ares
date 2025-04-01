use crate::{
    crypt::bitwise_xor,
    device::{Device, IndexData, MetaData, Model},
    form::Form,
    log::Log,
};
use regex::Regex;
use reqwest::{Client, header};
use scraper::{Html, Selector};
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

impl Device<'_> {
    fn handle_login_input_mitra_lc(&self) -> Result<(String, String), Box<dyn Error>> {
        todo!()
    }

    fn handle_login_input_askey_lc(
        &self,
        login_username: &str,
    ) -> Result<(String, String), Box<dyn Error>> {
        Ok((login_username.to_string(), self.admin_password.to_string()))
    }

    fn handle_login_input_mitra_econet(
        &self,
        login_username: &str,
    ) -> Result<(String, String), Box<dyn Error>> {
        let login_username = login_username.to_string();

        let login_password = format!("{:?}", md5::compute(self.admin_password.as_bytes()));

        Ok((login_username, login_password))
    }

    fn handle_login_input_askey_econet(
        &self,
        login_username: &str,
    ) -> Result<(String, String), Box<dyn Error>> {
        let login_username = bitwise_xor(login_username).unwrap_or_default();

        let login_password = bitwise_xor(self.admin_password).unwrap_or_default();

        Ok((login_username, login_password))
    }

    fn handle_login_input(&self, login_username: &str) -> Result<(String, String), Box<dyn Error>> {
        match self.model {
            Model::MitraLC => self.handle_login_input_mitra_lc(),
            Model::AskeyLC => self.handle_login_input_askey_lc(login_username),
            Model::MitraEconet => self.handle_login_input_mitra_econet(login_username),
            Model::AskeyEconet => self.handle_login_input_askey_econet(login_username),
            _ => {
                unreachable!()
            }
        }
    }

    fn generate_login_form<'a>(
        &self,
        (login_username, login_password): (String, String),
    ) -> Result<Form, Box<dyn Error>> {
        let mut login_form = HashMap::new();
        let target_uri;

        match self.model {
            Model::MitraLC => {
                todo!()
            }
            Model::AskeyLC => {
                login_form.insert("loginUsername", login_username);
                login_form.insert("loginPassword", login_password);
                login_form.insert("curWebPage", "/login.html".to_string());
                target_uri = "/login.cgi";
            }
            Model::MitraEconet => {
                todo!()
            }
            Model::AskeyEconet => {
                login_form.insert("loginUsername", login_username);
                login_form.insert("loginPassword", login_password);
                login_form.insert("curWebPage", "/index_cliente.asp".to_string());
                target_uri = "/cgi-bin/te_acceso_router.cgi";
            }
            _ => {
                unreachable!()
            }
        }

        Ok(Form::new(login_form, target_uri))
    }

    fn collect_variables_from_response(
        &self,
        uri: &str,
        response: String,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut vars = HashMap::new();

        match uri {
            "/index_cliente.asp" => {
                let fetch_pattern = Regex::new(
                    r#"var\s+(gponUp|opticalPower|pppStatus|pppIpv4Gateway|enetStatus|wlEnbl_main0|wlSsid_main0|wlEnbl_main1|wlSsid_main1)\s*=\s*['"]?([^"']+)['"]?;"#,
                )?;

                for capture in fetch_pattern.captures_iter(&*response) {
                    let key = capture[1].to_string();
                    let value = capture[2].to_string();
                    vars.insert(key, value);
                }
            }
            "/about-power-box.asp" => {
                let document = Html::parse_fragment(&*response);
                let td_selector = Selector::parse("td").unwrap();

                let mut key = None;

                for element in document.select(&td_selector) {
                    let text = element
                        .text()
                        .collect::<Vec<_>>()
                        .join("")
                        .trim()
                        .to_string();

                    if key.is_none() && text.ends_with(':') {
                        key = Some(text.trim_end_matches(':').to_string());
                    } else if let Some(k) = key.take() {
                        let value = if k == "Endereço MAC da WAN" {
                            text.replace(":", "") // Remove colons from MAC address
                        } else {
                            text
                        };
                        vars.insert(k, value);
                    }
                }
            }
            _ => {
                unreachable!()
            }
        }

        Ok(vars)
    }

    pub async fn login_to_index<'a>(self, client: &Client) -> Result<Self, Box<dyn Error>> {
        let login_form = self
            .generate_login_form(self.handle_login_input("admin").unwrap_or_default())
            .unwrap_or_default();

        let index_login_get_uri: &str = match self.model {
            Model::MitraLC => {
                todo!()
            }
            Model::AskeyLC => "",
            Model::MitraEconet => {
                todo!()
            }
            Model::AskeyEconet => "/login.asp",
        };

        let index_login_get_url = format!("http://{}{}", self.ip_addr, index_login_get_uri);

        // GET index login page to capture cookies
        let index_login_get_response = client
            .get(&index_login_get_url)
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
            )
            .header(header::REFERER, &index_login_get_url)
            .send()
            .await
            .expect("Device did not respond.");

        let index_login_post_url = format!("http://{}{}", self.ip_addr, login_form.target_uri);

        let login_data = self.handle_login_input("admin").unwrap_or_default();
        let login_form = self.generate_login_form(login_data).unwrap_or_default();

        let index_login_post_response = client
            .post(&index_login_post_url)
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
            )
            .header(header::REFERER, &index_login_post_url)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&login_form.form)
            .send()
            .await
            .expect("Device did not respond.");

        Ok(self)
    }

    pub async fn fetch_index_data(mut self, client: &Client) -> Result<Self, Box<dyn Error>> {
        let index_data_get_uri = "/index_cliente.asp";
        let index_data_get_url = format!("http://{}{}", self.ip_addr, index_data_get_uri);
        let index_data_get_response = client.get(&index_data_get_url).send().await?;

        if !index_data_get_response.status().is_success() {
            self.log(
                "IndexData",
                String::from("Fetch error. Response was not successful"),
            )?;
        } else {
            self.index_data = IndexData::from_hashmap(self.collect_variables_from_response(
                index_data_get_uri,
                index_data_get_response.text().await?,
            )?);

            dbg!(&self.index_data);

            self.log("IndexData", serde_json::to_string(&self.index_data)?)?;
        }

        Ok(self)
    }

    pub async fn fetch_meta_data(mut self, client: &Client) -> Result<Self, Box<dyn Error>> {
        let meta_data_get_uri = "/about-power-box.asp";
        let meta_data_get_url = format!("http://{}{}", self.ip_addr, meta_data_get_uri);
        let meta_data_get_response = client.get(&meta_data_get_url).send().await?;

        if !meta_data_get_response.status().is_success() {
            self.log(
                "MetaData",
                String::from("Fetch error. Response was not successful"),
            )?;
        } else {
            self.meta_data = MetaData::from_hashmap(self.collect_variables_from_response(
                meta_data_get_uri,
                meta_data_get_response.text().await?,
            )?);

            dbg!(&self.meta_data);

            self.log("MetaData", serde_json::to_string(&self.meta_data)?)?;
        }

        Ok(self)
    }
}
