use crate::{
    crypt,
    device::{Device, IndexData, MetaData, Model},
    fetch::{form::Form, variant::Variant},
};
use html_escape::decode_html_entities;
use regex::Regex;
use reqwest::{Client, header};
use scraper::{Html, Selector};
use std::collections::HashMap;
use unescape::unescape;

mod form;
mod variant;

impl Device {
    fn handle_login_input_mitra_lc(
        &self,
        _login_username: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        todo!()
    }

    fn generate_login_form_mitra_lc(
        &self,
        (_login_username, _login_password): (String, String),
    ) -> Result<Form, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn login_to_index_mitra_lc(
        self,
        _client: &Client,
        _login_form: Form,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn fetch_index_data_mitra_lc(
        self,
        _client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn fetch_meta_data_mitra_lc(
        self,
        _client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }

    fn collect_variables_from_response_mitra_lc(
        &self,
        _uri: &str,
        _response: String,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn handle_login_input_askey_lc(
        &self,
        login_username: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        Ok((login_username.to_string(), self.admin_password.to_string()))
    }

    fn generate_login_form_askey_lc(
        &self,
        (_login_username, _login_password): (String, String),
    ) -> Result<Form, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn login_to_index_askey_lc(
        self,
        _client: &Client,
        _login_form: Form,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn fetch_index_data_askey_lc(
        self,
        _client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn fetch_meta_data_askey_lc(
        self,
        _client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }

    fn collect_variables_from_response_askey_lc(
        &self,
        _uri: &str,
        _response: String,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn handle_login_input_mitra_econet(
        &self,
        login_username: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let login_username = login_username.to_string();

        let login_password = format!("{:x}", md5::compute(self.admin_password.as_bytes()));

        Ok((login_username, login_password))
    }

    fn generate_login_form_mitra_econet(
        &self,
        (_login_username, _login_password): (String, String),
    ) -> Result<Form, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn login_to_index_mitra_econet(
        self,
        _client: &Client,
        _login_form: Form,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn fetch_index_data_mitra_econet(
        self,
        _client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn fetch_meta_data_mitra_econet(
        self,
        _client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }

    fn collect_variables_from_response_mitra_econet(
        &self,
        _uri: &str,
        _response: String,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn handle_login_input_askey_econet(
        &self,
        login_username: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let login_username = crypt::bitwise_xor(login_username).unwrap_or_default();

        let login_password = crypt::bitwise_xor(&*self.admin_password).unwrap_or_default();

        Ok((login_username, login_password))
    }

    fn generate_login_form_askey_econet(
        &self,
        (login_username, login_password): (String, String),
    ) -> Result<Form, Box<dyn std::error::Error>> {
        let mut login_form = HashMap::new();
        let target_uri;

        login_form.insert("loginUsername".to_string(), login_username);
        login_form.insert("loginPassword".to_string(), login_password);
        login_form.insert("curWebPage".to_string(), "/index_cliente.asp".to_string());
        target_uri = "/cgi-bin/te_acceso_router.cgi";

        Ok(Form::new(login_form, target_uri))
    }

    async fn login_to_index_askey_econet(
        self,
        client: &Client,
        login_form: Form,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let index_login_get_uri = "/login.asp";
        let index_login_get_url = format!("http://{}{}", self.ip_address, index_login_get_uri);

        // GET index login page to capture cookies
        let _index_login_get_response = client
            .get(&index_login_get_url)
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
            )
            .header(header::REFERER, &index_login_get_url)
            .send()
            .await
            .map_err(|_| {
                println!("[90]: Device did not respond or incorrect password");
            });

        let index_login_post_url = format!("http://{}{}", self.ip_address, login_form.target_uri);

        let _index_login_post_response = client
            .post(&index_login_post_url)
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
            )
            .header(header::REFERER, &index_login_post_url)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&login_form.form)
            .send()
            .await?;

        Ok(self)
    }

    async fn fetch_index_data_askey_econet(
        mut self,
        client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let index_data_get_uri = "/index_cliente.asp";
        let index_data_get_url = format!("http://{}{}", self.ip_address, index_data_get_uri);
        let index_data_get_response = client.get(&index_data_get_url).send().await?;

        self.index_data = IndexData::from_hashmap(self.collect_variables_from_response(
            index_data_get_uri,
            index_data_get_response.text().await?,
        )?);

        Ok(self)
    }

    async fn fetch_meta_data_askey_econet(
        mut self,
        client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let meta_data_get_uri = "/about-power-box.asp";
        let meta_data_get_url = format!("http://{}{}", self.ip_address, meta_data_get_uri);
        let meta_data_get_response = client.get(&meta_data_get_url).send().await?;

        self.meta_data = MetaData::from_hashmap(self.collect_variables_from_response(
            meta_data_get_uri,
            meta_data_get_response.text().await?,
        )?);

        Ok(self)
    }

    fn collect_variables_from_response_askey_econet(
        &self,
        uri: &str,
        response: String,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut vars = HashMap::new();

        match uri {
            "/index_cliente.asp" => {
                let fetch_pattern = Regex::new(r#"\bvar\s+(\w+)\s*=\s*'([^']*)'"#)?;

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
            _ => (),
        }

        Ok(vars)
    }

    async fn handle_login_input_mitra_wifi6(
        &self,
        login_username: &str,
        client: &Client,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let index_login_get_uri = "/cgi-bin/login.cgi";
        let index_login_get_url = format!("http://{}/cgi-bin/login.cgi", &self.ip_address);

        let index_login_get_response = client
            .get(&index_login_get_url)
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
            )
            .header(header::REFERER, &index_login_get_url)
            .send()
            .await
            .map_err(|_| {
                println!("[90]: Device did not respond or incorrect password");
            })
            .unwrap()
            .text()
            .await?;

        let login_username = login_username.to_string();

        let sid = self
            .collect_variables_from_response_mitra_wifi6(
                &index_login_get_uri,
                index_login_get_response,
            )?
            .single_or_default();

        let login_password = format!(
            "{:x}",
            md5::compute(format!("{}:{}", self.admin_password, &sid))
        );

        Ok((login_username, login_password))
    }

    fn generate_login_form_mitra_wifi6(
        &self,
        (login_username, login_password): (String, String),
    ) -> Result<Form, Box<dyn std::error::Error>> {
        let mut login_form = HashMap::new();
        let target_uri = "/cgi-bin/login.cgi";

        login_form.insert("Loginuser".to_string(), login_username);
        login_form.insert("LoginPasswordValue".to_string(), login_password);
        login_form.insert("acceptLoginIndex".to_string(), "1".to_string());

        Ok(Form::new(login_form, target_uri))
    }

    async fn login_to_index_mitra_wifi6(
        self,
        client: &Client,
        login_form: Form,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let index_login_post_url = format!("http://{}{}", &self.ip_address, login_form.target_uri);

        let _index_login_post_response = client
            .post(&index_login_post_url)
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
            )
            .header(header::REFERER, &index_login_post_url)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&login_form.form)
            .send()
            .await?;

        Ok(self)
    }

    async fn fetch_index_data_mitra_wifi6(
        self,
        client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let index_info_get_uri = "/cgi-bin/sophia_info.cgi";
        let index_info_get_url = format!("http://{}{}", self.ip_address, &index_info_get_uri);

        let index_info_get_response = client
            .get(index_info_get_url)
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
            )
            .header(header::REFERER, "http://192.168.15.1/cgi-bin/login.cgi")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .send()
            .await?
            .text_with_charset("utf-8")
            .await?;

        // dbg!(&index_info_get_response);

        let vars = self
            .collect_variables_from_response_mitra_wifi6(
                "/cgi-bin/sophia_info.cgi",
                index_info_get_response,
            )?
            .multiple_or_default();

        dbg!(&vars);

        Ok(self)
    }

    async fn fetch_meta_data_mitra_wifi6(
        self,
        _client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }

    fn collect_variables_from_response_mitra_wifi6(
        &self,
        uri: &str,
        response: String,
    ) -> Result<Variant<String, HashMap<String, String>>, Box<dyn std::error::Error>> {
        match uri {
            "/cgi-bin/login.cgi" => {
                let fetch_pattern =
                    regex::Regex::new(r#"var\s+sid\s*=\s*['"]([a-fA-F0-9]+)['"]"#).unwrap();

                let capture = fetch_pattern
                    .captures(&response)
                    .and_then(|capture| capture.get(1).map(|m| m.as_str().to_string()))
                    .unwrap_or_default();

                Ok(Variant::Single(capture))
            }
            "/cgi-bin/sophia_info.cgi" => {
                // fn is_likely_key(text: &str) -> bool {
                //     text.ends_with(':')
                //         || [
                //             "GPON", "SSID", "PPP", "LAN1", "LAN2", "LAN3", "LAN4", "Wi-Fi",
                //         ]
                //         .contains(&text)
                // }
                //
                // fn is_likely_value(text: &str) -> bool {
                //     !text.ends_with(':') && !text.contains('<') && !text.is_empty()
                // }
                //
                // let mut vars = HashMap::new();
                //
                // let document = Html::parse_document(&response);
                // let key_selector = Selector::parse("strong, span")?;
                // let value_selector = Selector::parse("div, span, li")?;
                //
                // let mut keys = Vec::new();
                // let mut values = Vec::new();
                //
                // for element in document.select(&key_selector) {
                //     if let Some(text) = element.text().next() {
                //         let trimmed = text.trim();
                //         if !trimmed.is_empty() && is_likely_key(trimmed) {
                //             keys.push(trimmed.to_string());
                //         }
                //     }
                // }
                //
                // for element in document.select(&value_selector) {
                //     if let Some(text) = element.text().next() {
                //         let trimmed = text.trim();
                //         if !trimmed.is_empty() && is_likely_value(trimmed) {
                //             values.push(trimmed.to_string());
                //         }
                //     }
                // }
                //
                // for (key, value) in keys.into_iter().zip(values.into_iter()) {
                //     vars.insert(key, value);
                // }

                let mut vars = HashMap::new();

                let document = Html::parse_document(&response);

                let test = &document.html().to_string();

                let unescaped = unescape(test).unwrap_or_default();
                let decoded = decode_html_entities(&unescaped).to_string();

                let document = Html::parse_document(&decoded);

                dbg!(&document);

                // Helper selectors
                let span_selector = Selector::parse("span").unwrap();
                let strong_selector = Selector::parse("strong").unwrap();
                let div_selector = Selector::parse("div").unwrap();
                let li_selector = Selector::parse("li").unwrap();

                let spans: Vec<_> = document.select(&span_selector).collect();
                let strongs: Vec<_> = document.select(&strong_selector).collect();
                let divs: Vec<_> = document.select(&div_selector).collect();
                let lis: Vec<_> = document.select(&li_selector).collect();

                // Each (key, value) pair is extracted based on tag position (same as in reference document)
                let pairs = [
                    (spans.get(1), divs.get(3)),    // GPON -> Não Sincronizado
                    (strongs.get(1), divs.get(4)),  // Potência Rx -> 0 dBm
                    (strongs.get(2), divs.get(5)),  // Potência Tx -> 0 dBm
                    (strongs.get(3), spans.get(4)), // PPP -> Não Conectado
                    (spans.get(12), lis.get(6)),    // Endereço de IPv4 público -> 0.0.0.0
                    (strongs.get(4), Some(&strongs.get(4).unwrap())), // SSID -> VIVOFIBRA-WIFI6-D8F0
                    (spans.get(13), spans.get(14)),                   // Wi-Fi -> Ativado
                    (strongs.get(5), divs.get(6)),                    // LAN1 -> Unknown
                    (strongs.get(6), divs.get(7)),                    // LAN2 -> Unknown
                    (strongs.get(7), divs.get(8)),                    // LAN3 -> Unknown
                    (strongs.get(8), divs.get(9)),                    // LAN4 -> devenv
                ];

                for (key_opt, value_opt) in pairs {
                    if let (Some(key), Some(value)) = (key_opt, value_opt) {
                        let k = key.text().collect::<String>().trim().to_string();
                        let v = value.text().collect::<String>().trim().to_string();
                        vars.insert(k, v);
                    }
                }

                // let document = Html::parse_fragment(&*response);
                // let selector = Selector::parse("span").unwrap();
                //
                // let mut vars = HashMap::new();
                // let mut key = None;
                //
                // for element in document.select(&selector) {
                //     let text = element
                //         .text()
                //         .collect::<Vec<_>>()
                //         .join("")
                //         .trim()
                //         .to_string();
                //
                //     if key.is_none() && text.ends_with(':') {
                //         key = Some(text.trim_end_matches(':').to_string());
                //     } else if let Some(k) = key.take() {
                //         let value = if k == "Endereço MAC da WAN" {
                //             text.replace(":", "") // Remove colons from MAC address
                //         } else {
                //             text
                //         };
                //         vars.insert(k, value);
                //     }
                // }

                Ok(Variant::Multiple(vars))
            }
            _ => Ok(Variant::None),
        }
    }

    async fn generate_login_form(
        &self,
        login_username: &str,
        client: &Client,
    ) -> Result<Form, Box<dyn std::error::Error>> {
        match self.model {
            Model::MitraLC => {
                self.generate_login_form_mitra_lc(self.handle_login_input_mitra_lc(login_username)?)
            }
            Model::AskeyLC => {
                self.generate_login_form_askey_lc(self.handle_login_input_askey_lc(login_username)?)
            }
            Model::MitraEconet => self.generate_login_form_mitra_econet(
                self.handle_login_input_mitra_econet(login_username)?,
            ),
            Model::AskeyEconet => self.generate_login_form_askey_econet(
                self.handle_login_input_askey_econet(login_username)?,
            ),
            Model::MitraWiFi6 => self.generate_login_form_mitra_wifi6(
                self.handle_login_input_mitra_wifi6("admin", client).await?,
            ),
            Model::AskeyWiFi6 => self.generate_login_form_askey_econet(
                self.handle_login_input_askey_econet(login_username)?,
            ),
        }
    }

    fn collect_variables_from_response(
        &self,
        uri: &str,
        response: String,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        match self.model {
            Model::MitraLC => self.collect_variables_from_response_mitra_lc(uri, response),
            Model::AskeyLC => self.collect_variables_from_response_askey_lc(uri, response),
            Model::MitraEconet => self.collect_variables_from_response_mitra_econet(uri, response),
            Model::AskeyEconet => self.collect_variables_from_response_askey_econet(uri, response),
            // Model::MitraWiFi6 => self.collect_variables_from_response_mitra_wifi6(uri, response),
            Model::AskeyWiFi6 => self.collect_variables_from_response_askey_econet(uri, response),
            _ => unreachable!(),
        }
    }

    pub async fn login_to_index(self, client: &Client) -> Result<Self, Box<dyn std::error::Error>> {
        let login_form = self.generate_login_form("admin", client).await?;

        match self.model {
            Model::MitraLC => self.login_to_index_mitra_lc(client, login_form).await,
            Model::AskeyLC => self.login_to_index_askey_lc(client, login_form).await,
            Model::MitraEconet => self.login_to_index_mitra_econet(client, login_form).await,
            Model::AskeyEconet => self.login_to_index_askey_econet(client, login_form).await,
            Model::MitraWiFi6 => self.login_to_index_mitra_wifi6(client, login_form).await,
            Model::AskeyWiFi6 => self.login_to_index_askey_econet(client, login_form).await,
        }
    }

    pub async fn fetch_index_data(
        self,
        client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match self.model {
            Model::MitraLC => self.fetch_index_data_mitra_lc(client).await,
            Model::AskeyLC => self.fetch_index_data_askey_lc(client).await,
            Model::MitraEconet => self.fetch_index_data_mitra_econet(client).await,
            Model::AskeyEconet => self.fetch_index_data_askey_econet(client).await,
            Model::MitraWiFi6 => self.fetch_index_data_mitra_wifi6(client).await,
            Model::AskeyWiFi6 => self.fetch_index_data_askey_econet(client).await,
        }
    }

    pub async fn fetch_meta_data(
        self,
        client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match self.model {
            Model::MitraLC => self.fetch_meta_data_mitra_lc(client).await,
            Model::AskeyLC => self.fetch_meta_data_askey_lc(client).await,
            Model::MitraEconet => self.fetch_meta_data_mitra_econet(client).await,
            Model::AskeyEconet => self.fetch_meta_data_askey_econet(client).await,
            Model::MitraWiFi6 => self.fetch_meta_data_mitra_wifi6(client).await,
            Model::AskeyWiFi6 => self.fetch_meta_data_askey_econet(client).await,
        }
    }
}
