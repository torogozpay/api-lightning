use actix_web::{get, HttpResponse};

use shared::error_handler::CustomError;
use crate::utils::response as resp;


#[utoipa::path(
    get,
    path = "/api/lightning/v1/health_check",
    responses(
        (status = 200, description = "Testing", body = inline(resp::InvoiceResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    )
)]
#[get("/health_check")]
pub async fn get_test_handler() -> Result<HttpResponse, CustomError> {

    let info = "API Lightning Started".to_string();
 
    Ok(HttpResponse::Ok().json(info))        
}