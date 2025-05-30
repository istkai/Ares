use crate::device::Device;
use crate::fetch::form::Form;
use reqwest::Client;
use std::collections::HashMap;

pub(crate) fn handle_login_input(device: &Device, login_username: &str) -> (String, String) {
    (login_username.to_string(), device.admin_password.to_owned())
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
    _uri: &str,
    _response: String,
) -> HashMap<String, String> {
    todo!()
}

pub(crate) async fn login_to_index(
    _device: Device,
    _client: &Client,
) -> Result<Device, Box<dyn std::error::Error>> {
    todo!()
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
