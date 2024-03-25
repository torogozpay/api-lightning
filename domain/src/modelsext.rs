// domain/src/modelsext.rs

 use chrono::{Utc, DateTime};
 use serde::{Deserialize, Serialize}; 
 use utoipa::ToSchema;
 use bigdecimal::BigDecimal;
 
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct InvoiceData {
  pub business_id: i32,
  pub presell_id: i32,
  pub order_id: i32,
  pub split_id: i32,
  pub invoice_date: DateTime<Utc>,
  pub description: String,
  pub currency: String,
  pub total_amount: BigDecimal,
  pub amount_msat: i64,
  pub apply_split: bool,
}

 #[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
 pub struct InvoiceCheck {
  pub payment_request: String,
  pub amount: Option<u64>,
  pub fee: Option<u64>
 }

 #[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
 pub struct InvoiceFilters {
   pub hash: String 
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct Payment {
  pub address: String,
  pub amount: u64
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct PaymentFilters {
  pub address: String,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct InfoResponse {
    pub identity_pubkey: String,
    pub alias: Option<String>,
    pub block_height: String,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct InvoiceResponse {
    pub business_id: i32,
    pub order_id: i32,    
    pub payment_request: Option<String>,
    pub preimage: Option<String>,
    pub hash: Option<String>,
    pub paid: bool,
    pub status: String
}