#[warn(unused_assignments)]

use crate::settings::CONFIG;
use crate::error_handler::CustomError;
use actix_web::http::header::map::HeaderMap;
use base64::encode_config;
use std::time::SystemTime;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, errors::ErrorKind};
use domain::modelsext::Claims;
use base64;

use tracing::info;

pub struct JWT(pub Claims);

pub async fn verificate_token(headers: &HeaderMap) -> Result<i32, CustomError> {
      let header_value = headers.get("Authorization");
      let token = match header_value {
         Some(value) => {
               let val = value.to_str().unwrap_or_default().to_string();
               if val.starts_with("Bearer ") {
                   let token = val.replace("Bearer ","");
                   Some(token.to_string())
               } else {
                   None
               }
         }
         _ => None,
      };
   
      match &token {
         Some(token) => {
             // Print the token to the console to verify
             //info!("Received token: {}", token);
   
             let secret = CONFIG.jwt.jwt_secret.clone();
   
             match validate_token(&token, &secret) {
                 Ok(claims) => {
                     // Check other conditions according to your needs
                     let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize;
                     if claims.exp <= current_time {
                         Err(CustomError::new(401, "Not authorizated".to_string()))
                     } else { 
                         // You can add more validations here
                         Ok(claims.sub.trim().parse().expect("error business id!"))
                     }
                 }
                 Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
             }
         }
         None => Err(CustomError::new(401, "Not authorizated".to_string()))
     }
   
   }
   
   fn validate_token(token: &str, secret: &str) -> Result<Claims, CustomError> {
         // Configure validation options directly in Validation
         let mut validation = Validation::new(Algorithm::HS256);
         validation.leeway = 10;  // Allowed time setting for expiration times and not before
         validation.validate_exp = true;  //Validate expiration time (exp)
         validation.validate_nbf = true;  // Validate time before start (nbf)
         validation.validate_aud = false;  // Validate audience (aud)
     
         let decoding_key = DecodingKey::from_secret(secret.as_ref());
     
         let key_base64: String = encode_config(secret.as_bytes(), base64::STANDARD);
     
         info!("Decoding Key (Base64): {}", key_base64);
         //info!("Token: {}", token);
         //info!("Secret: {}", secret);
     
         match decode::<Claims>(token, &decoding_key, &validation) {
             Ok(token_data) => {
                 // Print iss and aud values
                 //info!("Issuer (iss): {}", token_data.claims.iss);
                 //info!("Subject (sub): {}", token_data.claims.sub);
                 //info!("Audience (aud): {}", token_data.claims.aud);
     
                 // Verify that the exp field is greater than the current time
                 let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize;
     
                 //info!("Token Time: {}", token_data.claims.exp);
     
                 if token_data.claims.exp <= current_time {
                     return Err(CustomError::new(401, "Not authorizated".to_string()));
                 }
     
                 // If valid, return the claims
                 Ok(token_data.claims)
             }
             Err(err) => {
                 // Handle decoding error
                 match err.kind() {
                     ErrorKind::ExpiredSignature => {
                         info!("Error decoding token: Token has expired");
                         return Err(CustomError::new(401, "Not authorizated".to_string()));
                     }
                     _ => {
                         info!("Error decoding token: {:?}", err);
                         return Err(CustomError::new(500, "Internal Server Error".to_string()));
                     }
                 }
             }
         }
     }
