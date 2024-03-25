// application/src/invoice/update.rs

use domain::models::Invoice;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;


pub async fn update_invoice(myinvoice: Invoice) -> Result<Invoice, CustomError> {
    use domain::schema::invoices::dsl::*;
    use domain::schema::invoices;

    let mut conn = db::connection()?;

    let mut myinvoice = myinvoice;
    myinvoice.updated_at = Some(chrono::offset::Utc::now());
    let id_invoice = myinvoice.id;
 
    let newinvoice = diesel::update(invoices.filter(invoices::id.eq(id_invoice))).set(&myinvoice).get_result::<Invoice>(&mut conn)?;
   
    Ok(newinvoice)            
}