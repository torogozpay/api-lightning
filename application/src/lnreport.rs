use domain::modelsext::{CreateToken,Token,ProcessPayment,ProcessResult};
use shared::{settings::CONFIG, error_handler::CustomError};
use reqwest::{header, Client};

use uuid::Uuid;
//use chrono::Utc;
use tracing::{info, error};


pub async fn api_business_auth() -> Result<String, CustomError>{
    let username = CONFIG.api.api_user.to_string();
    let password = CONFIG.api.api_pass.to_string(); 
    
    let socket: String;
    socket = CONFIG.api.api_server.to_string();

    let data_user = CreateToken {
       username: username,
       password: password
    };
      
    // Construct the request
    let auth_client = Client::builder().build()?; 
    let token = auth_client
            .post(socket.to_owned() + "/api/v1/authenticate")
            .header(header::CONTENT_TYPE, "application/json") 
            .json(&data_user)
            .send()
            .await?;

    // Check the response body
    let token_str = token.text().await?;
    //info!("Response Token: {:?}", token_str);
    
    // Deserialize JSON into struct
    let token: Result<Token, serde_json::Error> = serde_json::from_str(&token_str);

    match token { 
        Ok(token) => {
            //info!("Deserialized Token: {:?}", token);
            Ok(token.jwt)
        }
        Err(e) => {
            //error!("Error deserialized Token: {:?}", e);
            Err(CustomError::new(400, e.to_string()))
        }  
    }
}     

pub async fn process_order(jwt: String, presell_id: Uuid, lnaddress: String) -> Result<ProcessResult, CustomError> {
    let socket: String;
    socket = CONFIG.api.api_server.to_string();

    let json_pay = ProcessPayment {
        invoiceUid: presell_id.to_string(), 
        lnAddress: lnaddress,
    };

    // Construct the request
    let client = Client::builder().build()?; 
    let response = client
            .put(socket.to_owned() + "/api/v1/order/process/")
            .header("Authorization", format!("Bearer {}", jwt))
            .header(header::CONTENT_TYPE, "application/json") 
            .json(&json_pay)
            .send()
            .await?;

    // Check the response body
    let body = response.text().await?;
    //info!("Response Body: {:?}", body);
    
    // Deserialize JSON into struct
    let result: Result<ProcessResult, serde_json::Error> = serde_json::from_str(&body);

    match result {
        Ok(your_struct) => {
            info!("Deserialized process: {:?}", your_struct);
            Ok(your_struct)    
        }
        Err(e) => {
            error!("Error deserialized: {:?}", e);
            Err(CustomError::new(400, e.to_string()))
        }
    }

}    
