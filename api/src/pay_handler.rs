use application::lnurl;
use actix_web::{web, get, HttpRequest, HttpResponse};

use shared::{error_handler::CustomError, authorization::verify_auth};
use domain::models::Invoice;
use domain::modelsext::{Payment, PaymentFilters};
use crate::utils::response as resp;


#[utoipa::path(
    get,
    path = "/verifyAddress",
    responses(
        (status = 200, description = "Testing address", body = inline(Invoice)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    )
)]
#[get("/verifyAddress")]
pub async fn get_verify_address_handler(data: web::Json<PaymentFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_auth(req.headers()) {
        Ok(true) => {
        let result = lnurl::ln_exists(&data.address)
        .await
        .unwrap();
    
        Ok(HttpResponse::Ok().json(result))     
        
    },
    Ok(false) => Err(CustomError::new(401, "Not authorizated".to_string())),
    Err(_) => Err(CustomError::new(999, "Unknown error".to_string()))
    }      
}

#[utoipa::path(
    get,
    path = "/payment",
    responses(
        (status = 200, description = "Testing payment", body = inline(Invoice)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    )
)]
#[get("/payment")]
pub async fn get_payment_handler(data: web::Json<Payment>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_auth(req.headers()) {
        Ok(true) => {
        let result = lnurl::resolv_ln_address(&data.address, data.amount)
        .await
        .unwrap();
    
        Ok(HttpResponse::Ok().json(result))  
       
    },
    Ok(false) => Err(CustomError::new(401, "Not authorizated".to_string())),
    Err(_) => Err(CustomError::new(999, "Unknown error".to_string()))
    }         
}
