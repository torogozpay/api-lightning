use application::lnurl;
use actix_web::{web, post, HttpRequest, HttpResponse};

use shared::{error_handler::CustomError, authorization::verificate_token};
use domain::modelsext::{Payment, PaymentFilters};
use crate::utils::response as resp;


#[utoipa::path(
    post,
    path = "/api/lightning/v1/verifyAddress",
    responses(
        (status = 200, description = "Testing address", body = inline(resp::BoolResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/verifyAddress")]
pub async fn get_verify_address_handler(data: web::Json<PaymentFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verificate_token(req.headers()).await {
        Ok(_conf) => {
            let result = lnurl::ln_exists(&data.address)
            .await
            .unwrap();
        
            Ok(HttpResponse::Ok().json(result))     
        
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }      
}

#[utoipa::path(
    post,
    path = "/api/lightning/v1/generatePayInvoice",
    responses(
        (status = 200, description = "Generate new invoice by address", body = inline(resp::StringResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/generatePayInvoice")]
pub async fn get_payment_handler(data: web::Json<Payment>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verificate_token(req.headers()).await {
        Ok(_conf) => {
            let result = lnurl::resolv_ln_address(&data.address, data.amount, &data.description)
            .await
            .unwrap();
        
            Ok(HttpResponse::Ok().json(result))  
       
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }         
}