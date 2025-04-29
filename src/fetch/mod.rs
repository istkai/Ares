use crate::{
    device::{Device, Model},
    fetch::form::Form,
};
use reqwest::Client;
use std::collections::HashMap;

mod askey_econet;
mod askey_lc;
mod form;
mod mitra_econet;
mod mitra_lc;
mod mitra_wifi6;

impl Device {
    fn generate_login_form(&self, login_username: &str) -> Form {
        match self.model {
            Model::MitraLC => mitra_lc::generate_login_form(
                self,
                mitra_lc::handle_login_input(self, login_username),
            ),
            Model::AskeyLC => askey_lc::generate_login_form(
                self,
                askey_lc::handle_login_input(self, login_username),
            ),
            Model::MitraEconet => mitra_econet::generate_login_form(
                self,
                mitra_econet::handle_login_input(self, login_username),
            ),
            Model::AskeyEconet | Model::AskeyWiFi6 => askey_econet::generate_login_form(
                self,
                askey_econet::handle_login_input(self, login_username),
            ),
            Model::MitraWiFi6 => mitra_wifi6::generate_login_form(
                self,
                mitra_wifi6::handle_login_input(self, login_username),
            ),
        }
    }

    fn collect_variables_from_response(
        &self,
        uri: &str,
        response: String,
    ) -> HashMap<String, String> {
        match self.model {
            Model::MitraLC => mitra_lc::collect_variables_from_response(self, uri, response),
            Model::AskeyLC => askey_lc::collect_variables_from_response(self, uri, response),
            Model::MitraEconet => {
                mitra_econet::collect_variables_from_response(self, uri, response)
            }
            Model::AskeyEconet | Model::AskeyWiFi6 => {
                askey_econet::collect_variables_from_response(self, uri, response)
            }
            Model::MitraWiFi6 => mitra_wifi6::collect_variables_from_response(self, uri, response),
        }
    }

    pub async fn login_to_index(self, client: &Client) -> Result<Self, Box<dyn std::error::Error>> {
        match self.model {
            Model::MitraLC => mitra_lc::login_to_index(self, client).await,
            Model::AskeyLC => askey_lc::login_to_index(self, client).await,
            Model::MitraEconet => mitra_econet::login_to_index(self, client).await,
            Model::AskeyEconet | Model::AskeyWiFi6 => {
                askey_econet::login_to_index(self, client).await
            }
            Model::MitraWiFi6 => mitra_wifi6::login_to_index(self, client).await,
        }
    }

    pub async fn fetch_index_data(
        self,
        client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match self.model {
            Model::MitraLC => mitra_lc::fetch_index_data(self, client).await,
            Model::AskeyLC => askey_lc::fetch_index_data(self, client).await,
            Model::MitraEconet => mitra_econet::fetch_index_data(self, client).await,
            Model::AskeyEconet | Model::AskeyWiFi6 => {
                askey_econet::fetch_index_data(self, client).await
            }
            Model::MitraWiFi6 => mitra_wifi6::fetch_index_data(self, client).await,
        }
    }

    pub async fn fetch_meta_data(
        self,
        client: &Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match self.model {
            Model::MitraLC => mitra_lc::fetch_meta_data(self, client).await,
            Model::AskeyLC => askey_lc::fetch_meta_data(self, client).await,
            Model::MitraEconet => mitra_econet::fetch_meta_data(self, client).await,
            Model::AskeyEconet | Model::AskeyWiFi6 => {
                askey_econet::fetch_meta_data(self, client).await
            }
            Model::MitraWiFi6 => mitra_wifi6::fetch_meta_data(self, client).await,
        }
    }
}
