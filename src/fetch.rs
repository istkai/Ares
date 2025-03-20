use std::collections::HashMap;
use std::error::Error;
use reqwest::{header, Client};
use regex::Regex;
use md5;
use scraper::{Html, Selector};
use crate::{
    crypt::bitwise_xor,
    device::{IndexData, Device, MetaData},
    form::Form
};

impl Device<'_> {
    fn handle_login_input_mitra_lc(&self, login_username: &str) -> Result<(String, String), Box<dyn Error>> {
        todo!()
    }

    fn handle_login_input_askey_lc(&self, login_username: &str) -> Result<(String, String), Box<dyn Error>> {
        Ok(
            (login_username.to_string(), self.admin_password.to_string())
        )
    }

    fn handle_login_input_mitra_econet(&self, login_username: &str) -> Result<(String, String), Box<dyn Error>> {
        let login_username = login_username.to_string();

        let login_password = format!("{:?}", md5::compute(self.admin_password.as_bytes()));

        Ok(
            (login_username, login_password)
        )
    }

    fn handle_login_input_askey_econet(&self, login_username: &str) -> Result<(String, String), Box<dyn Error>> {
        let login_username = bitwise_xor(login_username).unwrap_or_default();

        let login_password = bitwise_xor(self.admin_password).unwrap_or_default();

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
                unreachable!()
            }
        }
    }

    fn generate_login_form<'a>(&self, (login_username, login_password): (String, String)) -> Option<Form> {
        let mut login_form = HashMap::new();
        let target_uri;

        match self.model {
            "Mitra-LC" => {
                return None
            },
            "Askey-LC" => {
                login_form.insert("loginUsername", login_username);
                login_form.insert("loginPassword", login_password);
                login_form.insert("curWebPage", "/login.html".to_string());
                target_uri = "/login.cgi";
            },
            "Mitra-Econet" => {
                return None
            },
            "Askey-Econet" => {
                login_form.insert("loginUsername", login_username);
                login_form.insert("loginPassword", login_password);
                login_form.insert("curWebPage", "/index_cliente.asp".to_string());
                target_uri = "/cgi-bin/te_acceso_router.cgi";
            },
            _ => {
                unreachable!()
            }
        }

        Some(Form::new(login_form, target_uri))
        
    }

    fn collect_variables_from_response(&self, uri: &str, response: &str) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        
        match uri {
            "/index_cliente.asp" => {
                let fetch_pattern = Regex::new(
                    r#"var\s+(gponUp|opticalPower|pppStatus|pppIpv4Gateway|enetStatus|wlEnbl_main0|wlSsid_main0|wlEnbl_main1|wlSsid_main1)\s*=\s*['"]?([^"']+)['"]?;"#)
                    .unwrap();

                for capture in fetch_pattern.captures_iter(response) {
                    let key = capture[1].to_string();
                    let value = capture[2].to_string();
                    vars.insert(key, value);
                }
            }
            "/about-power-box.asp" => {
                    let document = Html::parse_fragment(response);
                    let td_selector = Selector::parse("td").unwrap();
                
                    let mut key = None;

                    for element in document.select(&td_selector) {
                        let text = element
                            .text()
                            .collect::<Vec<_>>().join("")
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
        
        vars
        
    }

    pub async fn login_to_index<'a>(self, client: &Client) -> Result<Self, Box<dyn Error>> {
        let login_form = self
            .generate_login_form(
                self
                    .handle_login_input("admin").unwrap_or_default()).unwrap_or_default();
        
        let index_login_get_uri: &str = match self.model {
            "Mitra-LC" => {
                todo!()
            },
            "Askey-LC" => {
                ""
            },
            "Mitra-Econet" => {
                todo!()
            },
            "Askey-Econet" => {
                "/login.asp"
            },
            _ => {
                unreachable!()
            }
        };
        
        let index_login_get_url = format!("http://{}{}", self.ip_addr, index_login_get_uri);

        // GET index login page to capture cookies
        let index_login_get_response = client
            .get(&index_login_get_url)
            .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
            .header(header::REFERER, &index_login_get_url)
            .send()
            .await
            .expect("Device did not respond.");
        
        let index_login_post_url = format!("http://{}{}", self.ip_addr, login_form.target_uri);
        
        let login_data = self.handle_login_input("admin").unwrap_or_default();
        let login_form = self.generate_login_form(login_data).unwrap_or_default();

        let index_login_post_response = client
            .post(&index_login_post_url)
            .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0")
            .header(header::REFERER, &index_login_post_url)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&login_form.form)
            .send()
            .await
            .expect("Device did not respond.");

        dbg!(index_login_post_response.text().await.unwrap_or_default());

        Ok(self)
        
    }

    pub async fn fetch_index_data(&self, client: &Client) -> Result<IndexData, Box<dyn Error>> {
        let index_data_get_uri = "/index_cliente.asp";
        let index_data_get_url = format!("http://{}{}", self.ip_addr, index_data_get_uri);
        let index_data_get_response = self.collect_variables_from_response(
            index_data_get_uri,
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
    
    pub async fn fetch_meta_data(&self, client: &Client) -> Result<(MetaData), Box<dyn Error>> {
        let meta_data_get_uri = "/about-power-box.asp";
        let meta_data_get_url = format!("http://{}{}", self.ip_addr, meta_data_get_uri);
        let meta_data_get_response = self.collect_variables_from_response(
            meta_data_get_uri,
            client
                .get(&meta_data_get_url)
                .send()
                .await?
                .text()
                .await?
                .as_str()
        );
        
        let meta_data = MetaData::from_hashmap(meta_data_get_response);
        
        Ok(meta_data)
        
    }
}
