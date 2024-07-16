// domain/src/models.rs
#[warn(non_camel_case_types)]

use crate::schema::*;
use chrono::{Utc, DateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize}; 
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

use serde_with::{serde_as, DisplayFromStr};
use bigdecimal::BigDecimal;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::my_uuid;


 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Selectable, Insertable, AsChangeset, ToSchema, Clone)]
 #[diesel(table_name = invoices)]
 pub struct NewInvoice {
    pub business_id: i32,
    pub order_id: i32,
    #[schema(value_type = String)]
    #[serde(with = "my_uuid")]
    pub presell_id: Uuid,
    pub bolt11: String,
    pub payment_hash: Option<String>,
    pub payment_secret: Option<String>,
    pub description: String,
    pub customer_name: String,
    pub customer_email: String,
    pub currency: String,
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub sub_total: BigDecimal, 
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub taxes: BigDecimal, 
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub shipping: BigDecimal, 
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub total_amount: BigDecimal, 
    pub amount_sat: i32, 
    pub status: i32,
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub invoice_date: DateTime<Utc>,
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub updated_at: Option<DateTime<Utc>>,  
    pub distributed: bool,    
    pub apply_split: bool,    
 }
 
 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, ToSchema, Clone, Default)]
 #[diesel(table_name = invoices)]
 pub struct Invoice {
    pub id: i32,
    pub business_id: i32,
    pub order_id: i32,
    #[schema(value_type = String)]
    #[serde(with = "my_uuid")]    
    pub presell_id: Uuid,
    pub bolt11: String,
    pub payment_hash: Option<String>,
    pub payment_secret: Option<String>,
    pub description: String,
    pub customer_name: String,
    pub customer_email: String,
    pub currency: String,
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub sub_total: BigDecimal, 
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub taxes: BigDecimal, 
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub shipping: BigDecimal, 
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub total_amount: BigDecimal, 
    pub amount_sat: i32, 
    pub status: i32,
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub invoice_date: DateTime<Utc>,
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub updated_at: Option<DateTime<Utc>>,
    pub distributed: bool,
    pub apply_split: bool,
 }


 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Associations, ToSchema, Clone, Default)]
 #[diesel(belongs_to(Invoice, foreign_key = invoice_id))] 
 #[diesel(table_name = payment_split)]
 pub struct NewPaymentSplit {
   pub invoice_id: i32,
   pub tipo_asociado: String,
   pub lnaddress: String,
   pub amount_sat: i32,
   pub fee_sat: i32,
   pub status: i32,
   pub bolt11: Option<String>,
   pub payment_hash: Option<String>,
   pub payment_secret: Option<String>,
   pub attempts: i32,
   pub reported: bool,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Associations, ToSchema, Clone)]
#[diesel(belongs_to(Invoice, foreign_key = invoice_id))] 
#[diesel(table_name = payment_split)]
pub struct PaymentSplit {
  pub id: i32,
  pub invoice_id: i32,
  pub tipo_asociado: String,
  pub lnaddress: String,
  pub amount_sat: i32,
  pub fee_sat: i32,
  pub status: i32,
  pub bolt11: Option<String>,
  pub payment_hash: Option<String>,
  pub payment_secret: Option<String>,
  pub attempts: i32,
  pub reported: bool,
}