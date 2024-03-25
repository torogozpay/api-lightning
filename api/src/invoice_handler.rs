use application::{lightning_c, lightning_lnd};
use actix_web::{get, post, web, HttpRequest, HttpResponse};

use shared::{error_handler::CustomError, authorization::verify_auth};
use domain::models::{Invoice as MyInvoice, InvoiceFilters, InfoNode};
use domain::modelsext::{InvoiceResponse as MyInvoiceResponse,InfoResponse};
use crate::utils::response as resp;


#[utoipa::path(
    get,
    path = "/getInfo",
    responses(
        (status = 200, description = "Get info of node", body = inline(resp::InvoiceResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    )
)]
#[get("/getInfo")]
pub async fn get_info_handler(data: web::Json<InfoNode>) -> Result<HttpResponse, CustomError> {
    let info: InfoResponse;
    let data = data.into_inner();

    if data.lnd {
        info = lightning_lnd::LndConnector::getinfo(data)
                .await
                .unwrap();
    } else {
        info = lightning_c::ClnConnector::getinfo(data) 
                .await
                .unwrap();
    }

    Ok(HttpResponse::Ok().json(info))        
}

#[utoipa::path(
    post,
    path = "/createInvoice",
    responses(
        (status = 200, description = "Create a new invoice", body = inline(resp::InvoiceResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 999, description = "Unknown error", body = inline(resp::ErrorResponse))
    )
)]
#[post("/createInvoice")]
<<<<<<< HEAD
pub async fn create_invoice_handler(invoice : web::Json<InvoiceData>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_auth(req.headers()) {
        Ok(true) => {
            let inv = invoice.clone();
            let mut invoiceln = lightning::LndConnector::create_invoice(invoice.into_inner()).await.unwrap();

            invoiceln.business_id = inv.business_id;
            invoiceln.order_id = inv.order_id;

            match invoice::create::create_invoice(inv, invoiceln.clone()).await {
                Ok(_newinvoice) => {

                    Ok(HttpResponse::Ok().json(invoiceln))
                },
                Err(_) => Err(CustomError::new(994, "Invoice error".to_string()))
            }        
        },
        Ok(false) => Err(CustomError::new(401, "Not authorizated".to_string())),
        Err(_) => Err(CustomError::new(999, "Lightning error".to_string()))
    }
}



=======
pub async fn create_invoice_handler(invoice : web::Json<MyInvoice>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_auth(req.headers()) {
        Ok(true) => {
            let newinvoice: MyInvoiceResponse;
            let invoice = invoice.into_inner();
        
            if invoice.lnd {
                newinvoice = lightning_lnd::LndConnector::create_invoice(invoice)
                                .await
                                .unwrap();                
            } else {    
                newinvoice = lightning_c::ClnConnector::create_invoice(invoice)
                                .await
                                .unwrap();    
            }

            Ok(HttpResponse::Ok().json(newinvoice))
        },
        Ok(false) => Err(CustomError::new(401, "Not authorizated".to_string())),
        Err(_) => Err(CustomError::new(999, "Unknown error".to_string()))
    }
}

>>>>>>> 6c9e694fbf93fc24f09fd6128eca3c8b262613c2
#[utoipa::path(
    get,
    path = "/getInvoice",
    responses(
        (status = 200, description = "Get a invoice identifies with hash", body = inline(resp::InvoiceFiltersResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse)),
        (status = 404, description = "Invoice was not found", body = inline(resp::ErrorResponse)),
        (status = 999, description = "Unknown error", body = inline(resp::ErrorResponse))
    )
)]
#[get("/getInvoice")]
pub async fn get_invoice_handler(invoice_filters: web::Json<InvoiceFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_auth(req.headers()) {
        Ok(true) => {
<<<<<<< HEAD
            let newinvoice = lightning::LndConnector::get_invoice(invoice_filters.into_inner()).await.unwrap();
=======
            let newinvoice: MyInvoiceResponse;
            let invoice_filters = invoice_filters.into_inner();
        
            if invoice_filters.lnd {
                newinvoice = lightning_lnd::LndConnector::get_invoice(invoice_filters)
                                .await
                                .unwrap();
            } else {                
                newinvoice = lightning_c::ClnConnector::get_invoice(invoice_filters)
                                .await
                                .unwrap();
            }
>>>>>>> 6c9e694fbf93fc24f09fd6128eca3c8b262613c2

            Ok(HttpResponse::Ok().json(newinvoice))
        },
        Ok(false) => Err(CustomError::new(401, "Not authorizated".to_string())),
        Err(_) => Err(CustomError::new(999, "Unknown error".to_string()))
<<<<<<< HEAD
    }
}

#[utoipa::path(
    get,
    path = "/checkInvoice",
    responses(
        (status = 200, description = "Verify a invoice identifies with hash", body = inline(resp::InvoiceFiltersResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse)),
        (status = 404, description = "Invoice was not found", body = inline(resp::ErrorResponse)),
        (status = 999, description = "Unknown error", body = inline(resp::ErrorResponse))
    )
)]
#[get("/checkInvoice")]
pub async fn check_invoice_handler(invoice_data: web::Json<InvoiceCheck>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_auth(req.headers()) {
        Ok(true) => {
            let newinvoice = lightning::invoice::is_valid_invoice(invoice_data.into_inner()).unwrap();

            Ok(HttpResponse::Ok().json(newinvoice.to_string()))
       
        },
        Ok(false) => Err(CustomError::new(401, "Not authorizated".to_string())),
        Err(_) => Err(CustomError::new(999, "Unknown error".to_string()))
    }
=======
  }
>>>>>>> 6c9e694fbf93fc24f09fd6128eca3c8b262613c2
}