#[warn(unused_imports)]

use application::lightning::{LndConnector,invoice};
use application::invoice::{create, read, update::update_status_in_invoice};
use actix_web::{post, web, HttpRequest, HttpResponse};

use shared::{error_handler::CustomError, authorization::verificate_token};
use domain::models::Invoice;
use domain::modelsext::{DataInvoice, DataLookupInvoice, InvoiceData, InvoiceFilters, OrderFilters, InvoiceCheck, InvoiceResponse, LookupInvoiceResponse, OrderResponse};
use crate::utils::response as resp;
use uuid::Uuid;

use tracing::info;

#[utoipa::path(
    post,
    path = "/api/lightning/v1/createInvoice",
    responses(
        (status = 200, description = "Create a new invoice", body = inline(resp::InvoiceResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/createInvoice")]
pub async fn create_invoice_handler(invoice : web::Json<InvoiceData>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verificate_token(req.headers()).await {
        Ok(_conf) => {
            let rpc = LndConnector::new().await;
            let inv = invoice.clone();

            match rpc?.create_invoice(invoice.into_inner()).await {
                Ok(invoiceln) => {
                    match create::create_invoice(inv, invoiceln.clone()).await {
                        Ok(_new) => {
                            
                            let data_invoice = InvoiceResponse {
                                business_id: _new.business_id,
                                woocomerce_id: _new.order_id,    
                                tpay_preorder_id: _new.presell_id,    
                                invoice_request: invoiceln.invoice_request,
                                preimage: Some("".to_string()),
                                hash: invoiceln.hash,
                                status: invoiceln.status,
                                paid: false,
                                result: "success".to_string(),
                                code: 200,
                                message: "Invoice created".to_string()
                            };
                            
                            let data = DataInvoice {
                                data: data_invoice                    
                            };

                            Ok(HttpResponse::Ok().json(data))
                        },
                        Err(e) => Err(CustomError::new(994, e.to_string()))
                    }        
                }                
                Err(err) => {
                    println!("{:?}",err);

                    let data_invoice = InvoiceResponse {
                        business_id: 0,
                        woocomerce_id: 0,    
                        tpay_preorder_id: Uuid::new_v4(),    
                        invoice_request: "".to_string(),
                        preimage: Some("".to_string()),
                        hash: Some("".to_string()),
                        status: 0,
                        paid: false,
                        result: "failed".to_string(),
                        code: 400,
                        message: "Unable to create invoice".to_string()
                    };
                    
                    let data = DataInvoice {
                        data: data_invoice                    
                    };

                    Ok(HttpResponse::Ok().json(data))                
                }   
            }      
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }
}

#[utoipa::path(
    post,
    path = "/api/lightning/v1/lookupInvoice",
    responses(
        (status = 200, description = "Get a invoice identifies with hash", body = inline(resp::InvoiceFiltersResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse)),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/lookupInvoice")]
pub async fn get_invoice_handler(invoice_filters: web::Json<InvoiceFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verificate_token(req.headers()).await {
        Ok(_conf) => {
            let rpc = LndConnector::new().await;

            let filtr = invoice_filters.into_inner();
            match rpc?.get_invoice(filtr.clone()).await {
                Ok(invoiceln) => {
                    find_invoice_by_hash_in_db(filtr.hash, invoiceln).await
                },
                Err(_) => {            
                    let invoiceln: LookupInvoiceResponse = Default::default();
                    let _new: Invoice = Default::default();
                    Ok(HttpResponse::Ok().json(get_data_invoice(invoiceln, _new, true)))
                }   
            }                    
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }
}

pub async fn find_invoice_by_hash_in_db(hash: String, invoiceln: LookupInvoiceResponse) -> Result<HttpResponse, CustomError> {
    match read::get_invoice_by_hash(hash).await {
        Ok(_new) => {
            
            Ok(HttpResponse::Ok().json(get_data_invoice(invoiceln, _new, false)))
        },
        Err(_) => {
            let _new: Invoice = Default::default();
            Ok(HttpResponse::Ok().json(get_data_invoice(invoiceln, _new, false)))
        }   
    }   
}
pub fn get_data_invoice(invoiceln: LookupInvoiceResponse, _new: Invoice, err: bool) -> DataLookupInvoice {
    let data_invoice = LookupInvoiceResponse  {
        business_id: _new.business_id,
        woocomerce_id: _new.order_id,    
        tpay_preorder_id: _new.presell_id,    
        hash: _new.payment_hash,
        currency: _new.currency,
        totalAmount: _new.total_amount,
      
        memo: invoiceln.memo,
        r_preimage: invoiceln.r_preimage,
        r_hash: invoiceln.r_hash,   
        value: invoiceln.value,
        value_msat: invoiceln.value_msat,
        settled: invoiceln.settled,
        settle_date: invoiceln.settle_date,
        creation_date: invoiceln.creation_date,
        payment_request: invoiceln.payment_request,
        expiry: invoiceln.expiry,
        cltv_expiry: invoiceln.cltv_expiry,
        private: invoiceln.private,
        add_index: invoiceln.add_index,
        settle_index: invoiceln.settle_index,
        amt_paid: invoiceln.amt_paid,
        amt_paid_sat: invoiceln.amt_paid_sat,
        amt_paid_msat: invoiceln.amt_paid_msat,
        paid: invoiceln.paid,
        state: invoiceln.state,

        result: if !err {"success".to_string()} else {"failed".to_string()},
        code: if !err {200} else {400},
        message: if !err {"Invoice consulted".to_string()} else {"Unable to locate invoice".to_string()}
    };

    let data = DataLookupInvoice {
        data: data_invoice                    
    };

    return data;
}

#[utoipa::path(
    post,
    path = "/api/lightning/v1/checkInvoice",
    responses(
        (status = 200, description = "Verify a invoice identifies with address", body = inline(resp::InvoiceFiltersResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse)),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/checkInvoice")]
pub async fn check_invoice_handler(invoice_data: web::Json<InvoiceCheck>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verificate_token(req.headers()).await {
        Ok(_conf) => {
            match invoice::is_valid_invoice(invoice_data.into_inner()) {
                Ok(newinvoice) => {
                    Ok(HttpResponse::Ok().json(newinvoice.to_string()))
                },
                Err(err) => {
                    Err(CustomError::new(400, err.to_string()))
                }
            }
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }
}

#[utoipa::path(
    post,
    path = "/api/lightning/v1/lookupOrder",
    responses(
        (status = 200, description = "Get an order identifies with uuid", body = inline(resp::InvoiceFiltersResponse)),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse)),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/lookupOrder")]
pub async fn get_order_handler(order_filters: web::Json<OrderFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verificate_token(req.headers()).await {
        Ok(_conf) => {
            let order = order_filters.into_inner();
            match read::get_invoice_by_uuid(order.uuid).await {
                Ok(_new) => {
                    let mut order = get_data_order(_new, false);

                    let filter = InvoiceFilters {
                        hash: order.payment_hash.clone().expect("There's no hash")
                    };
                    
                    match update_status_in_invoice(filter).await {
                        Ok(paid) => {
                            if paid {
                                order.status = 1;
                            }
                        },
                        Err(err) => {
                            info!("Error changing status: {:?}", err.to_string())
                        }    
                    }

                    Ok(HttpResponse::Ok().json(order))
                },
                Err(_) => {
                    let _new: Invoice = Default::default();
                    Ok(HttpResponse::Ok().json(get_data_order(_new, true)))
                }    
            }   
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }
}

pub fn get_data_order(_new: Invoice, err: bool) -> OrderResponse {
    let data_invoice = OrderResponse  {
        id: _new.id,
        business_id: _new.business_id,
        order_id: _new.order_id,
        presell_id: _new.presell_id,
        bolt11: _new.bolt11,
        payment_hash: _new.payment_hash,
        payment_secret: _new.payment_secret,
        description: _new.description,
        customer_name: _new.customer_name,
        customer_email: _new.customer_email,
        currency: _new.currency,
        sub_total: _new.sub_total, 
        taxes: _new.taxes, 
        shipping: _new.shipping, 
        total_amount: _new.total_amount, 
        amount_sat: _new.amount_sat as i64, 
        status: _new.status,
        invoice_date: _new.invoice_date,
        created_at: _new.created_at,
        updated_at: _new.updated_at,
        distributed: _new.distributed,
        apply_split: _new.apply_split,
    
        result: if !err {"success".to_string()} else {"failed".to_string()},
        code: if !err {200} else {400},
        message: if !err {"Order consulted".to_string()} else {"Unable to locate order".to_string()}
    };

    return data_invoice;
}