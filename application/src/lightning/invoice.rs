use shared::settings::CONFIG;
use shared::error_handler::CustomError;
use anyhow::Result;

use domain::modelsext::InvoiceCheck;

use chrono::prelude::*;
use chrono::Duration;
use lightning_invoice::{Bolt11Invoice, SignedRawBolt11Invoice};
use std::str::FromStr;

use tracing::info;

/// Decode a lightning invoice (bolt11)
pub fn decode_invoice(payment_request: &str) -> Result<Bolt11Invoice, CustomError> {
    match Bolt11Invoice::from_str(payment_request) {
        Ok(invoice) =>  {
            Ok(invoice)
        }    
        Err(_) =>  {
            return Err(CustomError::new(400, "Failed decode invoice".to_string()))
        }    
    }            
}

/// Verify if a buyer invoice is valid,
/// if the invoice have amount we check if the amount minus fee is the same
pub fn is_valid_invoice(
    invoice_data: InvoiceCheck
) -> Result<Bolt11Invoice, CustomError> {
 
    match decode_invoice(&invoice_data.payment_request) {
        Ok(invoice) =>  {
            let amount_sat = invoice.amount_milli_satoshis().unwrap_or(0) / 1000;
            let fee = invoice_data.fee.unwrap_or(0);
        
            if let Some(amt) = invoice_data.amount {
                if amount_sat > 0 && amount_sat != (amt - fee) {
                    return Err(CustomError::new(400, "The amount on this invoice is wrong".to_string()))
                }
            }
        
            if invoice.is_expired() {
                return Err(CustomError::new(400, "Invoice has expired".to_string()))
             }
        
            match invoice_data.payment_request.parse::<SignedRawBolt11Invoice>() {
                Ok(parsed) =>  {            
                    let (parsed_invoice, _, _) = parsed.into_parts();
                
                    let expiration_window = CONFIG.node.expiry.clone() as i64;
                    let latest_date = Utc::now() + Duration::seconds(expiration_window);
                    let latest_date = latest_date.timestamp() as u64;
                    let expires_at =
                        invoice.expiry_time().as_secs() + parsed_invoice.data.timestamp.as_unix_timestamp();
                
                    if expires_at < latest_date {
                        return Err(CustomError::new(400, "Minimal expiration time on invoice".to_string()))
                    }
                
                    Ok(invoice)
                }    
                Err(_) =>  {
                    return Err(CustomError::new(400, "Failed parse invoice".to_string()))
                }  
            }           
        }    
        Err(_) =>  {
            return Err(CustomError::new(400, "Failed decode invoice".to_string()))
        }    
    }

}

pub fn is_expired_invoice(payment_request: &str) -> Result<bool, CustomError>{

    match decode_invoice(&payment_request) {
        Ok(invoice) =>  {    
            match payment_request.parse::<SignedRawBolt11Invoice>() {
                Ok(parsed) =>  {    
                    let (parsed_invoice, _, _) = parsed.into_parts();

                    let latest_date = Utc::now();
                    let latest_date = latest_date.timestamp() as u64;
                    let expires_at =
                        invoice.expiry_time().as_secs() + parsed_invoice.data.timestamp.as_unix_timestamp();
                
                    info!("Timestamp   = {:?}",Utc::now());
                    info!("Expires_at  = {:?}",expires_at);
                    info!("Latest_date = {:?}",latest_date);
                
                    let mut expired : bool = false;
                    if latest_date >= expires_at {
                        info!("Invoice expired = YES");
                        expired = true;
                    } else {
                        info!("Invoice expired = NOT");
                    }
                
                    Ok(expired)   
                }    
                Err(_) =>  {
                    return Err(CustomError::new(400, "Failed parse invoice".to_string()))
                }    
            } 
        }    
        Err(_) =>  {
            return Err(CustomError::new(400, "Failed decode invoice".to_string()))
        }    
    }  
}