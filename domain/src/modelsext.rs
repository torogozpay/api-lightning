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
 
/// Define a structure to represent data of the invoice
#[serde_as]   
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct InvoiceData {
  /// Represents the business identifier 
  pub business_id: i32,
  /// Represents the pre sale identifier 
  #[schema(value_type = String)]
  #[serde(with = "my_uuid")]   
  pub presell_id: Uuid,
  /// Represents the order‘s identifier 
  pub order_id: i32,
  /// Represents the invoice‘s date 
  #[schema(value_type = String)]
  pub invoice_date: DateTime<Utc>,
  /// Represents the invoice‘s description 
  pub description: String,
  /// Represents the customer‘s name 
  pub customer_name: String,
  /// Represents the email address
  pub customer_email: String,
  /// Represents the transaction‘s currency 
  pub currency: String,
  /// Represents the order‘s subtotal  
  #[schema(value_type = String)]
  pub sub_total: BigDecimal,
  /// Represents the order‘s taxes 
  #[schema(value_type = String)]
  pub taxes: BigDecimal,
  /// Represents the order‘s shipping 
  #[schema(value_type = String)]
  pub shipping: BigDecimal,
  /// Represents the order‘s total amount 
  #[schema(value_type = String)]
  pub total_amount: BigDecimal,
  /// Represents the order‘s total sats  
  pub amount_sat: u64,
  /// Represents split payment if applicable
  pub apply_split: bool,
  /// Represents the order detail
  pub paymentSplit: Vec<PreorderSplit>     
}

/// Define a structure to represent data of collaborators payments
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct PreorderSplit {
  /// Represents the pre sale identifier 
  #[schema(value_type = String)]
  pub invoiceUid: Uuid,  
  /// Represents the type of collaborator
  pub tipoAsociado: String,
  /// Represents the address LNURL of the collaborator
  pub ldAddress: String,
  /// Represents the amount (sats) to pay
  pub amountSat: i32,
  /// Represents the payment status 
  pub status: i32,  
  /// Represents the invoice address 
  pub invoiceAddress: String, 
  /// Represents the number of payment sending attempts
  pub attempts: i32,    
}

/// Define a structure to validate invoice
 #[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
 pub struct InvoiceCheck {
  /// Represents the invoice request 
  pub payment_request: String,
  /// Represents the invoice total amount
  pub amount: Option<u64>,
  /// Represents the invoice fee 
  pub fee: Option<u64>
 }

 /// Define a structure to filter the invoice by hash
 #[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
 pub struct InvoiceFilters {
   /// Represents the invoice hash 
   pub hash: String 
}

/// Define a structure to represent the filter of the order
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
 pub struct OrderFilters {
   /// Represents the order identifier 
   #[schema(value_type = String)]
   pub uuid: Uuid
}

/// Define a structure to represent the data of the payment
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct Payment {
  /// Represents the payment‘s LNURL 
  pub address: String,
  /// Represents the payment‘s total amount 
  pub amount: u64,
  /// Represents the payment‘s description 
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

/// Define a structure to represent the filter of the payment
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct PaymentFilters {
  /// Represents the payment‘s LNURL 
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