use application::lightning_lnd;
use actix_web::{get, post, web, HttpRequest, HttpResponse};

use shared::{error_handler::CustomError, authorization::verify_auth};
use domain::models::{Invoice as MyInvoice, InvoiceFilters};
use crate::utils::response as resp;

#[utoipa::path(
    post,
    path = "/lnd/createInvoice",
    responses(
        (status = 200, description = "Create a new invoice", body = inline(resp::InvoiceResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 999, description = "Unknown error", body = inline(resp::ErrorResponse))
    )
)]
#[post("/lnd/createInvoice")]
pub async fn create_invoice_handler(invoice : web::Json<MyInvoice>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_auth(req.headers()) {
        Ok(true) => {
            let connector = lightning_lnd::LndConnector::new(invoice.socket.clone(), invoice.cert.clone(), invoice.macaroon.clone()).await;
            let newinvoice = lightning_lnd::LndConnector::create_invoice(&mut lightning_lnd::LndConnector { client: connector.client }, invoice.into_inner())
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
    path = "/lnd/getInvoice",
    responses(
        (status = 200, description = "Get a invoice identifies with hash", body = inline(resp::InvoiceFiltersResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse)),
        (status = 404, description = "Invoice was not found", body = inline(resp::ErrorResponse)),
        (status = 999, description = "Unknown error", body = inline(resp::ErrorResponse))
    )
)]
#[get("/lnd/getInvoice")]
pub async fn get_invoice_handler(invoice_filters: web::Json<InvoiceFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_auth(req.headers()) {
        Ok(true) => {
            let connector = lightning_lnd::LndConnector::new(invoice_filters.socket.clone(), invoice_filters.cert.clone(), invoice_filters.macaroon.clone()).await;
            let newinvoice = lightning_lnd::LndConnector::get_invoice(&mut lightning_lnd::LndConnector { client: connector.client }, invoice_filters.into_inner()).await.unwrap();

            Ok(HttpResponse::Ok().json(newinvoice))
        },
        Ok(false) => Err(CustomError::new(401, "Not authorizated".to_string())),
        Err(_) => Err(CustomError::new(999, "Unknown error".to_string()))
    }
}