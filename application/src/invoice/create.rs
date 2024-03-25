// apllication/src/invoice/create.rs

use domain::models::{Invoice, NewInvoice, PaymentSplit, NewPaymentSplit};
use domain::modelsext::{InvoiceResponse, InvoiceData};
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use tracing::info;


pub async fn create_invoice(mydata: InvoiceData, myinvoice: InvoiceResponse) -> Result<Invoice, CustomError> { 
    use domain::schema::invoices;
    use domain::schema::payment_split;
    
    let mut conn = db::connection()?;
    
    let invoice = 
        NewInvoice {
            business_id: mydata.business_id,
            presell_id: mydata.presell_id,
            order_id: mydata.order_id,
            split_id: mydata.split_id,
            bolt11: myinvoice.payment_request,
            payment_hash: myinvoice.hash,
            payment_secret: myinvoice.preimage,
            description: mydata.description,
            currency: mydata.currency,
            total_amount: mydata.total_amount, 
            amount_msat: mydata.amount_msat as i32,
            status: "0".to_string(),
            invoice_date: mydata.invoice_date,
            created_at: chrono::offset::Utc::now(),   
            updated_at: Some(chrono::offset::Utc::now()),
            distributed: false,         
            apply_split: mydata.apply_split         
        };
    
    let newinvoice = diesel::insert_into(invoices::table).values(&invoice).get_result::<Invoice>(&mut conn)?;  

    info!("business_id {:?}",newinvoice.business_id.clone());
    info!("order_id {:?}",newinvoice.order_id.clone());
    info!("invoice_id {:?}",newinvoice.id.clone());

    /*-------------*/
    let split_listdets = create_payment_split(newinvoice.id.clone())?;
    let newsplit_listdets = diesel::insert_into(payment_split::table).values(&split_listdets).get_results::<PaymentSplit>(&mut conn)?;

    info!("split_listdets {:?}",newsplit_listdets);
    /*-------------*/

    Ok(newinvoice)
}

fn create_payment_split(invoice_id: i32) -> Result<Vec<NewPaymentSplit>, CustomError> {
    let pay1 = NewPaymentSplit {
        invoice_id: invoice_id.clone(),
        lnaddress: "torogozdev2023@blink.sv".to_string(),
        amount: BigDecimal::from_str("0.50").expect("error bigdecimal"),
        amount_msat: 1000,
        status: "0".to_string(),
        bolt11: Some("".to_string()),
        attempts: 0,
    };

    let pay2 = NewPaymentSplit {
        invoice_id: invoice_id.clone(),
        lnaddress: "torogozdev@zbd.gg".to_string(),
        amount: BigDecimal::from_str("0.25").expect("error bigdecimal"),
        amount_msat: 500,
        status: "0".to_string(),
        bolt11: Some("".to_string()),
        attempts: 0,
    };

    let mut split_listdets: Vec<NewPaymentSplit> = Vec::new();
    split_listdets.push(pay1);
    split_listdets.push(pay2);

    Ok(split_listdets)
}    