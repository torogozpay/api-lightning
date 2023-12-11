// domain/src/models.rs

 use serde::{Deserialize, Serialize}; 
 use utoipa::ToSchema;

 #[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct InfoNode {
  pub lnd: bool, 
  pub socket: String, 
  pub cert: String, 
  pub macaroon: String,  
  pub path: String
}

 #[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
 pub struct Invoice {
   pub lnd: bool, 
   pub socket: String, 
   pub cert: String, 
   pub macaroon: String,
   pub path: String,
   pub expiry: u32,
   pub cltv: u32,
   pub description: String,
   pub amount: i64
 }

 #[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
 pub struct InvoiceFilters {
   pub lnd: bool, 
   pub socket: String, 
   pub cert: String, 
   pub macaroon: String,  
   pub path: String,
   pub hash: String 
}