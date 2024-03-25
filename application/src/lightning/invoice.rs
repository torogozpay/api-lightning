use shared::settings::CONFIG;
use shared::error_handler::CustomError;
use anyhow::Result;

use domain::modelsext::InvoiceCheck;

use chrono::prelude::*;
use chrono::Duration;
use lightning_invoice::{Bolt11Invoice, SignedRawBolt11Invoice};
use std::str::FromStr;

/// Decode a lightning invoice (bolt11)
pub fn decode_invoice(payment_request: &str) -> Result<Bolt11Invoice, anyhow::Error> {
    let invoice = Bolt11Invoice::from_str(payment_request)
    .expect("Failed decode invoice");

    Ok(invoice)
}

/// Verify if a buyer invoice is valid,
/// if the invoice have amount we check if the amount minus fee is the same
pub fn is_valid_invoice(
    invoice_data: InvoiceCheck
) -> Result<Bolt11Invoice, CustomError> {
 
    let invoice = decode_invoice(&invoice_data.payment_request)
    .expect("Failed decode invoice");

    let amount_sat = invoice.amount_milli_satoshis().unwrap_or(0) / 1000;
    let fee = invoice_data.fee.unwrap_or(0);

    if let Some(amt) = invoice_data.amount {
        if amount_sat > 0 && amount_sat != (amt - fee) {
            return Err(CustomError::new(701, "The amount on this invoice is wrong".to_string()))
        }
    }

    if invoice.is_expired() {
        return Err(CustomError::new(702, "Invoice has expired".to_string()))
     }

    let parsed = invoice_data.payment_request.parse::<SignedRawBolt11Invoice>()
    .expect("Failed parse invoice");

    let (parsed_invoice, _, _) = parsed.into_parts();

    let expiration_window = CONFIG.node.expiry.clone() as i64;
    let latest_date = Utc::now() + Duration::seconds(expiration_window);
    let latest_date = latest_date.timestamp() as u64;
    let expires_at =
        invoice.expiry_time().as_secs() + parsed_invoice.data.timestamp.as_unix_timestamp();

    if expires_at < latest_date {
        return Err(CustomError::new(703, "Minimal expiration time on invoice".to_string()))
    }

    Ok(invoice)
}