// domain/src/models.rs

use crate::schema::*;
use chrono::{Utc, DateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize}; 
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

use serde_with::{serde_as, DisplayFromStr};
use bigdecimal::BigDecimal;
use utoipa::ToSchema;


 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Selectable, Insertable, AsChangeset/*, Associations*/, ToSchema, Clone)]
 #[diesel(table_name = invoices)]
 pub struct NewInvoice {
    pub business_id: i32,
    pub presell_id: i32,
    pub split_id: i32,
    pub order_id: i32,
    pub bolt11: Option<String>,
    pub payment_hash: Option<String>,
    pub payment_secret: Option<String>,
    pub description: String,
    pub currency: String,
    #[serde_as(as = "DisplayFromStr")]
    pub total_amount: BigDecimal, 
    pub amount_msat: i32, 
    pub status: String,
    #[serde_as(as = "DisplayFromStr")]
    pub invoice_date: DateTime<Utc>,
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,  
    pub distributed: bool,    
    pub apply_split: bool,    
 }
 
 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq/*, Associations*/, ToSchema, Clone)]
 #[diesel(table_name = invoices)]
 pub struct Invoice {
    pub id: i32,
    pub business_id: i32,
    pub presell_id: i32,
    pub split_id: i32,
    pub order_id: i32,
    pub bolt11: Option<String>,
    pub payment_hash: Option<String>,
    pub payment_secret: Option<String>,
    pub description: String,
    pub currency: String,
    #[serde_as(as = "DisplayFromStr")]
    pub total_amount: BigDecimal, 
    pub amount_msat: i32, 
    pub status: String,
    #[serde_as(as = "DisplayFromStr")]
    pub invoice_date: DateTime<Utc>,
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub distributed: bool,
    pub apply_split: bool,
 }


 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Associations, ToSchema, Clone)]
 #[diesel(belongs_to(Invoice, foreign_key = invoice_id))] 
 #[diesel(table_name = payment_split)]
 pub struct NewPaymentSplit {
   pub invoice_id: i32,
   pub lnaddress: String,
   pub amount: BigDecimal,
   pub amount_msat: i32,
   pub status: String,
   pub bolt11: Option<String>,
   pub attempts: i32,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Associations, ToSchema, Clone)]
#[diesel(belongs_to(Invoice, foreign_key = invoice_id))] 
#[diesel(table_name = payment_split)]
pub struct PaymentSplit {
  pub id: i32,
  pub invoice_id: i32,
  pub lnaddress: String,
  pub amount: BigDecimal,
  pub amount_msat: i32,
  pub status: String,
  pub bolt11: Option<String>,
  pub attempts: i32,
}
