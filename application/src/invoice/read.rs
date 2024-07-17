// application/src/invoice/read.rs

use domain::models::Invoice;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
use uuid::Uuid;

pub async fn get_invoice_by_hash(hash: String) -> Result<Invoice, CustomError> {
    use domain::schema::invoices;

    let mut conn = db::connection()?;

    let invoice = invoices::table.filter(invoices::payment_hash.eq(hash)).select(Invoice::as_select()).get_result(&mut conn)?;

    drop(conn);

    Ok(invoice)
}

pub async fn get_invoice_by_uuid(uuid: Uuid) -> Result<Invoice, CustomError> {
    use domain::schema::invoices;

    let mut conn = db::connection()?;

    let invoice = invoices::table.filter(invoices::presell_id.eq(uuid)).select(Invoice::as_select()).get_result(&mut conn)?;

    drop(conn);

    Ok(invoice)
}