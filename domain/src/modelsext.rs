// domain/src/modelsext.rs

 use serde::{Deserialize, Serialize}; 
 use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct InfoResponse {
    pub identity_pubkey: String,
    pub alias: Option<String>,
    pub block_height: String,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct InvoiceResponse {
    pub payment_request: Option<String>,
    pub preimage: Option<String>,
    pub hash: Option<String>,
    pub paid: bool,
    pub expires_at: Option<i64>,
    pub warnings: Option<String>,
}