// apllication/src/invoice/create.rs

use domain::models::{Invoice, NewInvoice, PaymentSplit, NewPaymentSplit};
use domain::modelsext::{InvoiceResponse, InvoiceData, PreorderSplit};
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
use tracing::info;


pub async fn create_invoice(mydata: InvoiceData, myinvoice: InvoiceResponse) -> Result<Invoice, CustomError> { 
    use domain::schema::invoices;
    use domain::schema::payment_split;
    
    let mut conn = db::connection()?;
    
    let invoice = 
        NewInvoice {
            business_id: mydata.business_id,
            order_id: mydata.order_id,
            presell_id: mydata.presell_id,
            bolt11: myinvoice.invoice_request,
            payment_hash: myinvoice.hash,
            payment_secret: myinvoice.preimage,
            description: mydata.description,
            customer_name: mydata.customer_name,
            customer_email: mydata.customer_email,
            currency: mydata.currency,
            sub_total: mydata.sub_total, 
            taxes: mydata.taxes, 
            shipping: mydata.shipping, 
            total_amount: mydata.total_amount, 
            amount_sat: mydata.amount_sat as i32,
            status: myinvoice.status,
            invoice_date: mydata.invoice_date,
            created_at: chrono::offset::Utc::now(),   
            updated_at: Some(chrono::offset::Utc::now()),
            distributed: false,         
            apply_split: mydata.apply_split         
        };
    
    let newinvoice = diesel::insert_into(invoices::table).values(&invoice).get_result::<Invoice>(&mut conn)?;  

    info!("business_id {:?}",newinvoice.business_id.clone());
    info!("order_id {:?}",newinvoice.order_id.clone());
    info!("presell_id {:?}",newinvoice.presell_id.clone());
    info!("invoice_id {:?}",newinvoice.id.clone());

    /*-------------*/
    let split_listdets = create_payment_split(newinvoice.id.clone(), mydata.paymentSplit)?;
    let newsplit_listdets = diesel::insert_into(payment_split::table).values(&split_listdets).get_results::<PaymentSplit>(&mut conn)?;

    info!("split_listdets {:?}",newsplit_listdets);
    /*-------------*/

    drop(conn);

    Ok(newinvoice)
}

fn create_payment_split(invoice_id: i32, listdets: Vec<PreorderSplit>) -> Result<Vec<NewPaymentSplit>, CustomError> {
    let mut split_listdets: Vec<NewPaymentSplit> = Vec::new();

    for det in listdets {
        let pay = NewPaymentSplit {
            invoice_id: invoice_id.clone(),
            tipo_asociado: det.tipoAsociado,
            lnaddress: det.ldAddress,
            amount_sat: det.amountSat,
            ..Default::default()
        };

        split_listdets.push(pay);
    }

    Ok(split_listdets)
}    