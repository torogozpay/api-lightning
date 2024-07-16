// domain/src/modelsext.rs
#![allow(non_snake_case)]

 use chrono::{Utc, DateTime};
 use serde::{Deserialize, Serialize}; 
 use serde_with::serde_as;
 use utoipa::ToSchema;
 use bigdecimal::BigDecimal;
 use uuid::Uuid;
 use crate::my_uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
}
 
#[serde_as]   
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct InvoiceData {
  pub business_id: i32,
  #[schema(value_type = String)]
  #[serde(with = "my_uuid")]   
  pub presell_id: Uuid,
  pub order_id: i32,
  #[schema(value_type = String)]
  pub invoice_date: DateTime<Utc>,
  pub description: String,
  pub customer_name: String,
  pub customer_email: String,
  pub currency: String,
  #[schema(value_type = String)]
  pub sub_total: BigDecimal,
  #[schema(value_type = String)]
  pub taxes: BigDecimal,
  #[schema(value_type = String)]
  pub shipping: BigDecimal,
  #[schema(value_type = String)]
  pub total_amount: BigDecimal,
  pub amount_sat: u64,
  pub apply_split: bool,
  pub paymentSplit: Vec<PreorderSplit>     
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct PreorderSplit {
  #[schema(value_type = String)]
  pub invoiceUid: Uuid,  
  pub tipoAsociado: String,
  pub ldAddress: String,
  pub amountSat: i32,
  pub status: i32,  
  pub invoiceAddress: String, 
  pub attempts: i32,    
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
 pub struct OrderFilters {
  #[schema(value_type = String)]
   pub uuid: Uuid
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct Payment {
  pub address: String,
  pub amount: u64,
  pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct CreateToken {
  pub username: String,
  pub password: String
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct Token {
  pub jwt: String,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct ProcessPayment {
  pub invoiceUid: String,
  pub lnAddress: String,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct ProcessResult {
  pub success: bool,
  pub message: String,
  pub data: Option<String>
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
pub struct DataInvoice {
    pub data: InvoiceResponse,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct InvoiceResponse {
    pub business_id: i32,
    pub woocomerce_id: i32,  
    #[schema(value_type = String)]  
    pub tpay_preorder_id: Uuid,    
    pub invoice_request: String,
    pub preimage: Option<String>,
    pub hash: Option<String>,
    pub paid: bool,
    pub status: i32,
    pub result: String,
    pub code: i32,
    pub message: String
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct DataLookupInvoice {
    pub data: LookupInvoiceResponse,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)] 
pub struct LookupInvoiceResponse {
  pub business_id: i32,
  pub woocomerce_id: i32,    
  #[schema(value_type = String)]
  pub tpay_preorder_id: Uuid,    
  pub hash: Option<String>,
  pub currency: String,
  #[schema(value_type = String)]
  pub totalAmount: BigDecimal,
  pub memo: String,
  pub r_preimage: String,
  pub r_hash: String,
  pub value: i64,
  pub value_msat: i64,
  pub settled: bool,
  pub settle_date: i64,
  pub creation_date: i64,
  pub payment_request: String,
  pub expiry: i64,
  pub cltv_expiry: u64,
  pub private: bool,
  pub add_index: u64,
  pub settle_index: u64,
  pub amt_paid: i64,
  pub amt_paid_sat: i64,
  pub amt_paid_msat: i64,
  pub state: i32,
  pub paid: bool,
  pub result: String,
  pub code: i32,
  pub message: String  
}

// List Payments Response
#[derive(Serialize, Deserialize)]
pub struct ListPaymentsResponse {
    pub payment_status: i32,
    pub payment_failure_status: i32,
    pub payment_message: String,
    pub first_index_offset: u64,
    pub last_index_offset: u64,
    pub total_num_payments: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Default)]
pub struct OrderResponse {
   pub id: i32,
   pub business_id: i32,
   pub order_id: i32,
   #[schema(value_type = String)]
   pub presell_id: Uuid,
   pub bolt11: String,
   pub payment_hash: Option<String>,
   pub payment_secret: Option<String>,
   pub description: String,
   pub customer_name: String,
   pub customer_email: String,
   pub currency: String,
   #[schema(value_type = String)]
   pub sub_total: BigDecimal, 
   #[schema(value_type = String)]
   pub taxes: BigDecimal, 
   #[schema(value_type = String)]
   pub shipping: BigDecimal, 
   #[schema(value_type = String)]
   pub total_amount: BigDecimal, 
   pub amount_sat: i64, 
   pub status: i32,
   #[schema(value_type = String)]
   pub invoice_date: DateTime<Utc>,
   #[schema(value_type = String)]
   pub created_at: DateTime<Utc>,
   #[schema(value_type = String)]
   pub updated_at: Option<DateTime<Utc>>,
   pub distributed: bool,
   pub apply_split: bool,
   pub result: String,
   pub code: i32,
   pub message: String  
}