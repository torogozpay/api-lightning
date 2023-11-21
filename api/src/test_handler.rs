use actix_web::{get, HttpResponse};

use shared::error_handler::CustomError;
use domain::models::Invoice;
use crate::utils::response as resp;



#[utoipa::path(
    get,
    path = "/getTest",
    responses(
        (status = 200, description = "Testing", body = inline(Invoice)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    )
)]
#[get("/getTest")]
pub async fn get_test_handler() -> Result<HttpResponse, CustomError> {

    let info: Invoice = { Invoice {
        socket: "".to_string(),
        macaroon: "".to_string(),
        cert: "".to_string(),
        path: "/root/.lightning/bitcoin".to_string(),
        expiry: 360,
        cltv: 100,
        amount: 100,
        description: "Pago".to_string()
    }};
 
    Ok(HttpResponse::Ok().json(info))        
}


