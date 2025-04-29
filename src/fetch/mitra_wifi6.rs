use crate::device::Device;
use crate::fetch::form::Form;
use md5;
use reqwest::{Client, header};
use std::collections::HashMap;

pub(crate) fn handle_login_input(device: &Device, login_username: &str) -> (String, String) {
    let login_username = login_username.to_string();
    let sid = "709f9050";

    let login_password = format!(
        "{:x}",
        md5::compute(format!("{}:{}", device.admin_password, sid))
    );

    (login_username, login_password)
}

pub(crate) fn generate_login_form(
    _device: &Device,
    (login_username, login_password): (String, String),
) -> Form {
    let mut login_form = HashMap::new();
    let target_uri = "/login.cgi";

    login_form.insert("Loginuser".to_string(), login_username);
    login_form.insert("LoginPasswordValue".to_string(), login_password);
    login_form.insert("acceptLoginIndex".to_string(), "1".to_string());

    Form::new(login_form, target_uri)
}

pub(crate) fn collect_variables_from_response(
    _device: &Device,
    _uri: &str,
    _response: String,
) -> HashMap<String, String> {
    todo!()
}

pub(crate) async fn login_to_index(
    device: Device,
    client: &Client,
) -> Result<Device, Box<dyn std::error::Error>> {
    let login_form = device.generate_login_form("admin");

    let index_login_get_url = format!("http://{}", &device.ip_address);

    let _index_login_get_response = client
        .get(&index_login_get_url)
        .header(
            header::USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
        )
        // .header(header::REFERER, &index_login_get_url)
        .send()
        .await
        .map_err(|_| {
            println!("[90]: Device did not respond or incorrect password");
        });

    let index_login_post_url = format!("http://{}{}", &device.ip_address, login_form.target_uri);

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
        .await?;

    dbg!(&index_login_post_response);

    // let _index_login_post_response = client
    //         .post(&index_login_post_url)
    //         .header(
    //             header::USER_AGENT,
    //             "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
    //         )
    //         .header(header::REFERER, &index_login_post_url)
    //         .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
    //         .form(&login_form.form)
    //         .send()
    //         .await?;

    dbg!(index_login_post_response.text().await?.to_string());

    Ok(device)
}
pub async fn fetch_index_data(
    mut _device: Device,
    _client: &Client,
) -> Result<Device, Box<dyn std::error::Error>> {
    Ok(_device)
}

pub async fn fetch_meta_data(
    mut _device: Device,
    _client: &Client,
) -> Result<Device, Box<dyn std::error::Error>> {
    Ok(_device)
}
