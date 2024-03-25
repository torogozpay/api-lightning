use application::lightning;
use actix_web::{get, HttpResponse};

use shared::error_handler::CustomError;
use crate::utils::response as resp;


#[utoipa::path(
    get,
    path = "/health_check",
    responses(
        (status = 200, description = "Testing", body = inline(resp::InvoiceResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    )
)]
#[get("/health_check")]
pub async fn get_test_handler() -> Result<HttpResponse, CustomError> {

    let info: Invoice = { Invoice {
        lnd: true,
        socket: "".to_string(),
        macaroon: "".to_string(),
        cert: "".to_string(),
        path: "".to_string(),
        expiry: 360,
        cltv: 144,
        amount: 1,
        description: "test".to_string()
    }};
 
    Ok(HttpResponse::Ok().json(info))        
}

#[utoipa::path(
    get,
    path = "/getInfo",
    responses(
        (status = 200, description = "Get info of node", body = inline(resp::InvoiceResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    )
)]
#[get("/getInfo")]
pub async fn get_info_handler() -> Result<HttpResponse, CustomError> {
    let info = lightning::LndConnector::getinfo() 
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(info))        
}