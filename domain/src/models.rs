// domain/src/models.rs

 use serde::{Deserialize, Serialize}; 
 use utoipa::ToSchema;

 
 #[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
 pub struct Invoice {
    pub socket: String, 
    pub cert: String, 
    pub macaroon: String,
    pub path: String,
    pub expiry: u32,
    pub cltv: u32,
    pub description: String,
    pub amount: i64, //u64
 }

 #[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
 pub struct InvoiceFilters {
   pub socket: String, 
   pub cert: String, 
   pub macaroon: String,  
   pub path: String,
   pub hash: String 
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct InfoNode {
  pub path: String
}


#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct InvoiceResponse {
    pub payment_request: String,
    pub preimage: Vec<u8>, 
    pub hash: Vec<u8>,
    pub description: String,
    pub paid: bool,
}
