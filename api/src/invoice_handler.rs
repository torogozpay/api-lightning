use application::lightning;
use cln_rpc::{model::requests::InvoiceRequest, model::requests::ListinvoicesRequest};
use cln_rpc::primitives::*;
use easy_hasher::easy_hasher::*;
use rand::{self, RngCore};
use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;


use shared::error_handler::CustomError;
use domain::models::{Invoice,InvoiceFilters};
use crate::utils::response as resp;



#[utoipa::path(
    get,
    path = "/getInfo",
    responses(
        (status = 200, description = "Get info of node", body = inline(Invoice)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    )
)]
#[get("/getInfo")]
pub async fn get_info_handler() -> Result<HttpResponse, CustomError> {

    let cn = lightning::ClnConnector::new().await;

    let info = lightning::ClnConnector::getinfo(&mut lightning::ClnConnector { sock: (cn.sock) })
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(info))        
}


#[utoipa::path(
    post,
    path = "/createInvoice",
    responses(
        (status = 200, description = "Create a new invoice", body = inline(resp::InvoiceResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse))
    )
)]
#[post("/createInvoice")]
pub async fn create_invoice_handler(invoice: web::Json<Invoice>) -> Result<HttpResponse, CustomError> {

    let cn = lightning::ClnConnector::new().await;

    let amount = AmountOrAny::Amount(Amount::from_msat(invoice.amount));
    let label = format!("{}", Uuid::new_v4());

    let mut preimage = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut preimage);
    let hash = raw_sha256(preimage.to_vec()).to_hex_string();

    let invoice = lightning::ClnConnector::create_invoice(&mut lightning::ClnConnector { sock: (cn.sock) }, InvoiceRequest {
        amount_msat: amount, 
        label: label,
        description: invoice.description.clone(),
        preimage: Some(hash),
        expiry: Some(invoice.expiry.into()),
        deschashonly: None,
        cltv: Some(invoice.cltv),
        fallbacks: None 
        })
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(invoice))
}


 
#[utoipa::path(
    get,
    path = "/getInvoice",
    responses(
        (status = 200, description = "Get a invoice identifies with hash", body = inline(InvoiceFilters)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 404, description = "Invoice was not found", body = inline(resp::ErrorResponse))
    )
)]
#[get("/getInvoice")]
pub async fn get_invoice_handler(invoice_filters: web::Json<InvoiceFilters>) -> Result<HttpResponse, CustomError> {

    let cn = lightning::ClnConnector::new().await;

    let invoice = lightning::ClnConnector::get_invoice(&mut lightning::ClnConnector { sock: (cn.sock) }, ListinvoicesRequest{
        index: None, 
        invstring: None,
        label: None,
        limit: None,
        offer_id: None,
        start: None,
        payment_hash: Some(invoice_filters.hash.to_string())})
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(invoice))    
}