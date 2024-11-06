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
    /// Represents the business identifier  
    pub business_id: i32,
    /// Represents the order identifier
    pub order_id: i32,
    /// Represents the pre order identifier
    #[schema(value_type = String)]
    #[serde(with = "my_uuid")]    
    pub presell_id: Uuid,
    /// Represents the invoice request 
    pub bolt11: String,
    /// Represents the payment hash
    pub payment_hash: Option<String>,
    /// Represents the invoice secret
    pub payment_secret: Option<String>,
    /// Represents the invoice‘s description 
    pub description: String,
    /// Represents the customer‘s name
    pub customer_name: String,
    /// Represents the email address
    pub customer_email: String,
    /// Represents the transaction currency 
    pub currency: String,
    /// Represents the order‘s subtotal 
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub sub_total: BigDecimal, 
    /// Represents the order‘s taxes 
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub taxes: BigDecimal, 
    /// Represents the order‘s shipping amount
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub shipping: BigDecimal, 
    /// Represents the order‘s total amount 
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub total_amount: BigDecimal, 
    /// Represents the order‘s total sats  
    pub amount_sat: i32, 
    /// Represents the invoice‘s status  
    pub status: i32,
    /// Represents the order‘s date 
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub invoice_date: DateTime<Utc>,
    /// Represents the order‘s creation date 
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: DateTime<Utc>,
    /// Represents the order‘s update date
    #[schema(value_type = String)]
    pub updated_at: Option<DateTime<Utc>>,
    /// Indicates whether the payment was distributed
    pub distributed: bool,
    /// Represents split payment if applicable
    pub apply_split: bool,

 }
 
  /// Define a structure to represent data of the invoice
 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, ToSchema, Clone, Default)]
 #[diesel(table_name = invoices)]
 pub struct Invoice {
    /// Represents the invoice ID 
    pub id: i32,
   /// Represents the business identifier  
   pub business_id: i32,
   /// Represents the order identifier
   pub order_id: i32,
   /// Represents the pre order identifier
   #[schema(value_type = String)]
   #[serde(with = "my_uuid")]    
   pub presell_id: Uuid,
   /// Represents the invoice request 
   pub bolt11: String,
   /// Represents the payment hash
   pub payment_hash: Option<String>,
   /// Represents the invoice secret
   pub payment_secret: Option<String>,
   /// Represents the invoice‘s description 
   pub description: String,
   /// Represents the customer‘s name
   pub customer_name: String,
   /// Represents the email address
   pub customer_email: String,
   /// Represents the transaction currency 
   pub currency: String,
   /// Represents the order‘s subtotal 
   #[schema(value_type = String)]
   #[serde_as(as = "DisplayFromStr")]
   pub sub_total: BigDecimal, 
   /// Represents the order‘s taxes 
   #[schema(value_type = String)]
   #[serde_as(as = "DisplayFromStr")]
   pub taxes: BigDecimal, 
   /// Represents the order‘s shipping amount
   #[schema(value_type = String)]
   #[serde_as(as = "DisplayFromStr")]
   pub shipping: BigDecimal, 
   /// Represents the order‘s total amount 
   #[schema(value_type = String)]
   #[serde_as(as = "DisplayFromStr")]
   pub total_amount: BigDecimal, 
   /// Represents the order‘s total sats  
   pub amount_sat: i32, 
   /// Represents the invoice‘s status  
   pub status: i32,
   /// Represents the order‘s date 
   #[schema(value_type = String)]
   #[serde_as(as = "DisplayFromStr")]
   pub invoice_date: DateTime<Utc>,
   /// Represents the order‘s creation date 
   #[schema(value_type = String)]
   #[serde_as(as = "DisplayFromStr")]
   pub created_at: DateTime<Utc>,
   /// Represents the order‘s update date
   #[schema(value_type = String)]
   pub updated_at: Option<DateTime<Utc>>,
   /// Indicates whether the payment was distributed
   pub distributed: bool,
   /// Represents split payment if applicable
   pub apply_split: bool,
 }

 /// Define a structure to represent data of the payment split
 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Associations, ToSchema, Clone, Default)]
 #[diesel(belongs_to(Invoice, foreign_key = invoice_id))] 
 #[diesel(table_name = payment_split)]
 pub struct NewPaymentSplit {
   /// Represents the invoice identifier  
   pub invoice_id: i32,
   /// Represents the type of collaborator 
   pub tipo_asociado: String,
   /// Represents the address LNURL of the collaborator
   pub lnaddress: String,
   /// Represents the amount (sats) to pay
   pub amount_sat: i32,
   /// Represents the fee (sats) to pay
   pub fee_sat: i32,
   /// Represents the payment status 
   pub status: i32,
   /// Represents the invoice request 
   pub bolt11: Option<String>,
   /// Represents the payment hash 
   pub payment_hash: Option<String>,
   /// Represents the invoice secret 
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
   /// Represents the payment identifier  
   pub id: i32,
  /// Represents the invoice identifier  
  pub invoice_id: i32,
  /// Represents the type of collaborator 
  pub tipo_asociado: String,
  /// Represents the address LNURL of the collaborator
  pub lnaddress: String,
  /// Represents the amount (sats) to pay
  pub amount_sat: i32,
  /// Represents the fee (sats) to pay
  pub fee_sat: i32,
  /// Represents the payment status 
  pub status: i32,
  /// Represents the invoice request 
  pub bolt11: Option<String>,
  /// Represents the payment hash 
  pub payment_hash: Option<String>,
  /// Represents the invoice secret 
  pub payment_secret: Option<String>,
  /// Represents the number of payment sending attempts
  pub attempts: i32,
  /// Indicates whether the payment was reported
  pub reported: bool,
}