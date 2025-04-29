use crate::crypt::bitwise_xor;
use crate::device::{Device, IndexData, MetaData};
use crate::fetch::form::Form;
use regex::Regex;
use reqwest::{Client, header};
use scraper::{Html, Selector};
use std::collections::HashMap;

pub(crate) fn handle_login_input(device: &Device, login_username: &str) -> (String, String) {
    let login_username = bitwise_xor(login_username).unwrap_or_default();

    let login_password = bitwise_xor(&*device.admin_password).unwrap_or_default();

    (login_username, login_password)
}

pub(crate) fn generate_login_form(
    _device: &Device,
    (login_username, login_password): (String, String),
) -> Form {
    let mut login_form = HashMap::new();
    let target_uri;

    login_form.insert("loginUsername".to_string(), login_username);
    login_form.insert("loginPassword".to_string(), login_password);
    login_form.insert("curWebPage".to_string(), "/login.html".to_string());
    target_uri = "/login.cgi";

    Form::new(login_form, target_uri)
}

pub(crate) fn collect_variables_from_response(
    _device: &Device,
    uri: &str,
    response: String,
) -> HashMap<String, String> {
    let mut vars = HashMap::new();

    match uri {
        "/index_cliente.asp" => {
            let fetch_pattern =
                Regex::new(r#"\bvar\s+(\w+)\s*=\s*'([^']*)'"#).expect("[-1]: Unexpected error");

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

    dbg!(&vars);

    vars
}

pub(crate) async fn login_to_index(
    device: Device,
    client: &Client,
) -> Result<Device, Box<dyn std::error::Error>> {
    let login_form = device.generate_login_form("admin");

    let index_login_get_uri = "/login.asp";

    let index_login_get_url = format!("http://{}{}", &device.ip_address, index_login_get_uri);

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

    let index_login_post_url = format!("http://{}{}", &device.ip_address, login_form.target_uri);

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

    Ok(device)
}

pub async fn fetch_index_data(
    mut device: Device,
    client: &Client,
) -> Result<Device, Box<dyn std::error::Error>> {
    let index_data_get_uri = "/index_cliente.asp";
    let index_data_get_url = format!("http://{}{}", device.ip_address, index_data_get_uri);
    let index_data_get_response = client.get(&index_data_get_url).send().await?.text().await?;

    device.index_data = IndexData::from_hashmap(
        device.collect_variables_from_response(index_data_get_uri, index_data_get_response),
    );

    Ok(device)
}

pub async fn fetch_meta_data(
    mut device: Device,
    client: &Client,
) -> Result<Device, Box<dyn std::error::Error>> {
    let meta_data_get_uri = "/about-power-box.asp";
    let meta_data_get_url = format!("http://{}{}", device.ip_address, meta_data_get_uri);
    let meta_data_get_response = client.get(&meta_data_get_url).send().await?;

    device.meta_data =
        MetaData::from_hashmap(device.collect_variables_from_response(
            meta_data_get_uri,
            meta_data_get_response.text().await?,
        ));

    Ok(device)
}
