use lazy_static::lazy_static;
use actix_web::http::header::map::HeaderMap;
use base64;

use crate::settings;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("Config can be loaded");
}


pub fn verify_auth(headers: &HeaderMap) -> Result<bool, String> {
   let api_key = CONFIG.api.api_key.clone();
   let username = CONFIG.api.api_username.clone();
   let password = CONFIG.api.api_password.clone();

   let auth_string = format!("{}:{}", username, password);
   let auth_encoded= "Basic ".to_owned() + &base64::encode(&auth_string);

   let mut valid = false;

   let api_header= headers.get("x-api-key").unwrap().to_str().unwrap();
   let auth_header= headers.get("Authorization").unwrap().to_str().unwrap();

    if api_key == api_header && auth_encoded == auth_header {   
          valid = true;
    }

    Ok(valid)
}