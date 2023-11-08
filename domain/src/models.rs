// domain/src/models.rs

 use serde::{Deserialize, Serialize}; 
 use utoipa::ToSchema;

 
 #[derive(Debug, Serialize, Deserialize, ToSchema)]
 pub struct Invoice {
    pub expiry: u32,
    pub cltv: u32,
    pub description: String,
    pub amount: u64, 
 }

 #[derive(Debug, Serialize, Deserialize, ToSchema)]
 pub struct InvoiceFilters {
    pub hash: String,
 }