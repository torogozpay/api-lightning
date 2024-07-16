// application/src/invoice/update.rs

use domain::models::Invoice;
use domain::modelsext::InvoiceFilters;
use crate::lightning::LndConnector;

use infrastructure as db;
use diesel::prelude::*;
use shared::{error_handler::CustomError, functions};

use tracing::{info, error};

use domain::schema::invoices;
use domain::schema::invoices::dsl::*;


pub async fn update_invoice(myinvoice: Invoice) -> Result<Invoice, CustomError> {
    let mut conn = db::connection()?;

    let mut myinvoice = myinvoice;
    myinvoice.updated_at = Some(chrono::offset::Utc::now());
    let id_invoice = myinvoice.id;
 
    let newinvoice = diesel::update(invoices.filter(invoices::id.eq(id_invoice))).set(&myinvoice).get_result::<Invoice>(&mut conn)?;
   
    drop(conn);

    Ok(newinvoice)            
}

pub async fn update_status_in_invoice(filter: InvoiceFilters) -> Result<bool, CustomError> {
    let mut conn = db::connection()?;
    let rpc = LndConnector::new().await;

    let mut paid: bool = false;

    match rpc?.get_invoice(filter).await {
        Ok(re) => {
            paid = re.paid;
            if re.paid {
                diesel::update(invoices.filter(payment_hash.eq(functions::base64_to_hex(re.r_hash))))
                .set((payment_secret.eq(re.r_preimage), status.eq(re.state)))
                .execute(&mut conn)
                .expect("Error updating invoice");
            } else {
                diesel::update(invoices.filter(payment_hash.eq(functions::base64_to_hex(re.r_hash))))
                .set(status.eq(re.state))
                .execute(&mut conn)
                .expect("Error updating invoice");
            }
            info!("Job1 - Updating invoice {:?}", re.paid)
        },
        Err(err) => error!("Job1 - Error: {:?}", err)
    }

    drop(conn);

    Ok(paid)
}
