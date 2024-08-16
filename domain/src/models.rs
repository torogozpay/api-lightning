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

 /// Define a structure to represent data of the invoice
 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Selectable, Insertable, AsChangeset, ToSchema, Clone)]
 #[diesel(table_name = invoices)]
 pub struct NewInvoice {
    /// Represents the identifier of the business 
    pub business_id: i32,
    /// Represents the identifier of the order
    pub order_id: i32,
    /// Represents the identifier of the pre sale
    #[schema(value_type = String)]
    #[serde(with = "my_uuid")]    
    pub presell_id: Uuid,
    /// Represents the request of the invoice
    pub bolt11: String,
    /// Represents the hash of the payment
    pub payment_hash: Option<String>,
    /// Represents the secret of the invoice
    pub payment_secret: Option<String>,
    /// Represents the description of the invoice
    pub description: String,
    /// Represents the name of the customer
    pub customer_name: String,
    /// Represents the email of the email
    pub customer_email: String,
    /// Represents the currency of the transaction
    pub currency: String,
    /// Represents the subtotal of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub sub_total: BigDecimal, 
    /// Represents the taxes of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub taxes: BigDecimal, 
    /// Represents the shipping of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub shipping: BigDecimal, 
    /// Represents the total amount of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub total_amount: BigDecimal, 
    /// Represents the total sats of the order 
    pub amount_sat: i32, 
    /// Represents the status of the invoice 
    pub status: i32,
    /// Represents the date of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub invoice_date: DateTime<Utc>,
    /// Represents the creation date of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: DateTime<Utc>,
    /// Represents the update date of the order
    #[schema(value_type = String)]
    pub updated_at: Option<DateTime<Utc>>,
    /// Indicates whether the payment was distributed
    pub distributed: bool,
    /// Represents whether split payment applies
    pub apply_split: bool,

 }
 
  /// Define a structure to represent data of the invoice
 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, ToSchema, Clone, Default)]
 #[diesel(table_name = invoices)]
 pub struct Invoice {
    /// Represents the identifier of the invoice 
    pub id: i32,
    /// Represents the identifier of the business 
    pub business_id: i32,
    /// Represents the identifier of the order
    pub order_id: i32,
    /// Represents the identifier of the pre sale
    #[schema(value_type = String)]
    #[serde(with = "my_uuid")]    
    pub presell_id: Uuid,
    /// Represents the request of the invoice
    pub bolt11: String,
    /// Represents the hash of the payment
    pub payment_hash: Option<String>,
    /// Represents the secret of the invoice
    pub payment_secret: Option<String>,
    /// Represents the description of the invoice
    pub description: String,
    /// Represents the name of the customer
    pub customer_name: String,
    /// Represents the email of the email
    pub customer_email: String,
    /// Represents the currency of the transaction
    pub currency: String,
    /// Represents the subtotal of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub sub_total: BigDecimal, 
    /// Represents the taxes of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub taxes: BigDecimal, 
    /// Represents the shipping of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub shipping: BigDecimal, 
    /// Represents the total amount of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub total_amount: BigDecimal, 
    /// Represents the total sats of the order 
    pub amount_sat: i32, 
    /// Represents the status of the invoice 
    pub status: i32,
    /// Represents the date of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub invoice_date: DateTime<Utc>,
    /// Represents the creation date of the order
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: DateTime<Utc>,
    /// Represents the update date of the order
    #[schema(value_type = String)]
    pub updated_at: Option<DateTime<Utc>>,
    /// Indicates whether the payment was distributed
    pub distributed: bool,
    /// Represents whether split payment applies
    pub apply_split: bool,
 }

 /// Define a structure to represent data of the payment split
 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Associations, ToSchema, Clone, Default)]
 #[diesel(belongs_to(Invoice, foreign_key = invoice_id))] 
 #[diesel(table_name = payment_split)]
 pub struct NewPaymentSplit {
   /// Represents the identifier of the invoice 
   pub invoice_id: i32,
   /// Represents the type of collaborator 
   pub tipo_asociado: String,
   /// Represents the address LNURL of the collaborator
   pub lnaddress: String,
   /// Represents the amount (sats) to pay
   pub amount_sat: i32,
   /// Represents the fee (sats) to pay
   pub fee_sat: i32,
   /// Represents the status of the payment
   pub status: i32,
   /// Represents the request of the invoice
   pub bolt11: Option<String>,
   /// Represents the hash of the payment
   pub payment_hash: Option<String>,
   /// Represents the secret of the invoice
   pub payment_secret: Option<String>,
   /// Represents the number of payment sending attempts
   pub attempts: i32,
   /// Indicates whether the payment was reported
   pub reported: bool,
}

 /// Define a structure to represent data of the payment split
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Associations, ToSchema, Clone)]
#[diesel(belongs_to(Invoice, foreign_key = invoice_id))] 
#[diesel(table_name = payment_split)]
pub struct PaymentSplit {
   /// Represents the identifier of the payment 
   pub id: i32,
   /// Represents the identifier of the invoice 
   pub invoice_id: i32,
   /// Represents the type of collaborator 
   pub tipo_asociado: String,
   /// Represents the address LNURL of the collaborator
   pub lnaddress: String,
   /// Represents the amount (sats) to pay
   pub amount_sat: i32,
   /// Represents the fee (sats) to pay
   pub fee_sat: i32,
   /// Represents the status of the payment
   pub status: i32,
   /// Represents the request of the invoice
   pub bolt11: Option<String>,
   /// Represents the hash of the payment
   pub payment_hash: Option<String>,
   /// Represents the secret of the invoice
   pub payment_secret: Option<String>,
   /// Represents the number of payment sending attempts
   pub attempts: i32,
   /// Indicates whether the payment was reported
   pub reported: bool,
}