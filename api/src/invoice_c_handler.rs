use application::lightning_c;
use actix_web::{get, post, web, HttpRequest, HttpResponse};

use shared::{error_handler::CustomError, authorization::verify_auth};
use domain::models::{Invoice as MyInvoice, InvoiceFilters, InfoNode};
use crate::utils::response as resp;


#[utoipa::path(
    get,
    path = "/c/getInfo",
    responses(
        (status = 200, description = "Get info of node", body = inline(resp::InvoiceResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    )
)]
#[get("/c/getInfo")]
pub async fn get_info_handler(data: web::Json<InfoNode>) -> Result<HttpResponse, CustomError> {
    let info = lightning_c::ClnConnector::getinfo(data.into_inner()) 
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(info))        
}

#[utoipa::path(
    post,
    path = "/c/createInvoice",
    responses(
        (status = 200, description = "Create a new invoice", body = inline(resp::InvoiceResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 999, description = "Unknown error", body = inline(resp::ErrorResponse))
    )
)]
#[post("/c/createInvoice")]
pub async fn create_invoice_handler(invoice : web::Json<MyInvoice>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_auth(req.headers()) {
        Ok(true) => {
            let newinvoice = lightning_c::ClnConnector::create_invoice(invoice.into_inner())
                .await
                .unwrap();    
        
            Ok(HttpResponse::Ok().json(newinvoice))
        },
        Ok(false) => Err(CustomError::new(401, "Not authorizated".to_string())),
        Err(_) => Err(CustomError::new(999, "Unknown error".to_string()))
    }
}

#[utoipa::path(
    get,
    path = "/c/getInvoice",
    responses(
        (status = 200, description = "Get a invoice identifies with hash", body = inline(resp::InvoiceFiltersResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse)),
        (status = 404, description = "Invoice was not found", body = inline(resp::ErrorResponse)),
        (status = 999, description = "Unknown error", body = inline(resp::ErrorResponse))
    )
)]
#[get("/c/getInvoice")]
pub async fn get_invoice_handler(invoice_filters: web::Json<InvoiceFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_auth(req.headers()) {
        Ok(true) => {
            let newinvoice = lightning_c::ClnConnector::get_invoice(invoice_filters.into_inner())
                .await
                .unwrap();
            
            Ok(HttpResponse::Ok().json(newinvoice))
        },
        Ok(false) => Err(CustomError::new(401, "Not authorizated".to_string())),
        Err(_) => Err(CustomError::new(999, "Unknown error".to_string()))
  }
}