use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represent a base64 encoded string.
pub type Base64String = String;

/// Represent the possible states of an invoice.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum InvoiceState {
    /// The invoice is open and awaiting payment.
    OPEN = 0,
    /// The invoice has been settled and the payment has been confirmed.
    SETTLED = 1,
    /// The invoice has been canceled and is no longer valid.
    CANCELED = 2,
    /// The invoice has been accepted but not yet settled.
    ACCEPTED = 3,
    /// The invoice has been expired but not yet settled.
    EXPIRED = 4,
}

/// Represent the possible statuses of a payment.
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[allow(nonstandard_style)]
pub enum PaymentStatus {
    /// The payment status is unknown.
    UNKNOWN = 0,
    /// The payment is currently in flight.
    IN_FLIGHT = 1,
    /// The payment completed successfully.
    SUCCEEDED = 2,
    /// The payment failed.
    FAILED = 3,
}

/// Represent the possible failure reasons of a payment.
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[allow(nonstandard_style)]
pub enum PaymentFailureReason {
    /// Payment isn't failed (yet).
    FAILURE_REASON_NONE = 0,
    /// There are more routes to try, but the payment timeout was exceeded.
    FAILURE_REASON_TIMEOUT = 1,
    /// All possible routes were tried and failed permanently. Or were no routes to the destination at all.
    FAILURE_REASON_NO_ROUTE = 2,
    /// A non-recoverable error has occured.
    FAILURE_REASON_ERROR = 3,
    /// Payment details incorrect (unknown hash, invalid amt or invalid final cltv delta).
    FAILURE_REASON_INCORRECT_PAYMENT_DETAILS = 4,
    /// Insufficient local balance.
    FAILURE_REASON_INSUFFICIENT_BALANCE = 5,
}

/// Represent the possible statuses of an HTLCAttempt.
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[allow(nonstandard_style)]
pub enum HTLCStatus {
    /// The HTLC is currently in flight.
    IN_FLIGHT = 0,
    /// The HTLC completed successfully.
    SUCCEEDED = 1,
    /// The HTLC failed.
    FAILED = 2,
}

/// Represent the possible failure reasons of an HTLCAttempt.
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[allow(nonstandard_style)]
pub enum FailureCode {
    /// Reserved failure reason.
    RESERVED = 0,
    /// Incorrect or unknown payment details.
    INCORRECT_OR_UNKNOWN_PAYMENT_DETAILS = 1,
    /// Incorrect payment amount.
    INCORRECT_PAYMENT_AMOUNT = 2,
    /// Final incorrect CLTV expiry.
    FINAL_INCORRECT_CLTV_EXPIRY = 3,
    /// Final incorrect HTLC amount.
    FINAL_INCORRECT_HTLC_AMOUNT = 4,
    /// Final expiry too soon.
    FINAL_EXPIRY_TOO_SOON = 5,
    /// Invalid realm.
    INVALID_REALM = 6,
    /// Expiry too soon.
    EXPIRY_TOO_SOON = 7,
    /// Invalid onion version.
    INVALID_ONION_VERSION = 8,
    /// Invalid onion HMAC.
    INVALID_ONION_HMAC = 9,
    /// Invalid onion key.
    INVALID_ONION_KEY = 10,
    /// Amount below minimum.
    AMOUNT_BELOW_MINIMUM = 11,
    /// Fee insufficient.
    FEE_INSUFFICIENT = 12,
    /// Incorrect CLTV expiry.
    INCORRECT_CLTV_EXPIRY = 13,
    /// Channel disabled.
    CHANNEL_DISABLED = 14,
    /// Temporary channel failure.
    TEMPORARY_CHANNEL_FAILURE = 15,
    /// Required node feature missing.
    REQUIRED_NODE_FEATURE_MISSING = 16,
    /// Required channel feature missing.
    REQUIRED_CHANNEL_FEATURE_MISSING = 17,
    /// Unknown next peer.
    UNKNOWN_NEXT_PEER = 18,
    /// Temporary node failure.
    TEMPORARY_NODE_FAILURE = 19,
    /// Permanent node failure.
    PERMANENT_NODE_FAILURE = 20,
    /// Permanent channel failure.
    PERMANENT_CHANNEL_FAILURE = 21,
    /// Expiry too far.
    EXPIRY_TOO_FAR = 22,
    /// MPP timeout.
    MPP_TIMEOUT = 23,
    /// Invalid onion payload.
    INVALID_ONION_PAYLOAD = 24,
    /// Internal failure.
    INTERNAL_FAILURE = 997,
    /// Unknown failure.
    UNKNOWN_FAILURE = 998,
    /// An unreadable failure result is returned if the received failure message cannot be decrypted.
    UNREADABLE_FAILURE = 999,
}

/// Represent the possible failure reasons of an HTLCAttempt.
/// See [LND API documentation](https://api.lightning.community/api/lnd/router/send-payment-v2/index.html#lnrpcfeaturebit).  
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[allow(nonstandard_style)]
pub enum FeatureBit {
    /// Request for dataloss protection.
    DATALOSS_PROTECT_REQ = 0,
    /// Optional dataloss protection.
    DATALOSS_PROTECT_OPT = 1,
    /// Initial routing sync request.
    INITIAL_ROUING_SYNC = 3,
    /// Request for an upfront shutdown script.
    UPFRONT_SHUTDOWN_SCRIPT_REQ = 4,
    /// Optional upfront shutdown script.
    UPFRONT_SHUTDOWN_SCRIPT_OPT = 5,
    /// Request for gossip queries.
    GOSSIP_QUERIES_REQ = 6,
    /// Optional gossip queries.
    GOSSIP_QUERIES_OPT = 7,
    /// Request for TLV onion.
    TLV_ONION_REQ = 8,
    /// Optional TLV onion.
    TLV_ONION_OPT = 9,
    /// Request for extended gossip queries.
    EXT_GOSSIP_QUERIES_REQ = 10,
    /// Optional extended gossip queries.
    EXT_GOSSIP_QUERIES_OPT = 11,
    /// Request for static remote key.
    STATIC_REMOTE_KEY_REQ = 12,
    /// Optional static remote key.
    STATIC_REMOTE_KEY_OPT = 13,
    /// Request for payment address.
    PAYMENT_ADDR_REQ = 14,
    /// Optional payment address.
    PAYMENT_ADDR_OPT = 15,
    /// Request for multi-path payments.
    MPP_REQ = 16,
    /// Optional multi-path payments.
    MPP_OPT = 17,
    /// Request for wumbo channels.
    WUMBO_CHANNELS_REQ = 18,
    /// Optional wumbo channels.
    WUMBO_CHANNELS_OPT = 19,
    /// Request for anchor commitments.
    ANCHORS_REQ = 20,
    /// Optional anchor commitments.
    ANCHORS_OPT = 21,
    /// Request for anchor commitments with zero fee HTLC support.
    ANCHORS_ZERO_FEE_HTLC_REQ = 22,
    /// Optional anchor commitments with zero fee HTLC support.
    ANCHORS_ZERO_FEE_HTLC_OPT = 23,
    /// Request for AMP (Atomic Multi-Path) payments.
    AMP_REQ = 30,
    /// Optional AMP (Atomic Multi-Path) payments.
    AMP_OPT = 31,
}


/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/add-invoice#lnrpcinvoice).
#[derive(Debug, Default, Serialize)]
pub struct AddInvoiceRequest {
    pub memo: Option<String>,
    pub r_preimage: Option<String>,
    pub value: u64,
    pub description_hash: Option<String>,
    pub expiry: i32,
    pub fallback_addr: Option<String>,
    pub cltv_expiry: Option<i32>,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/add-invoice#lnrpcaddinvoiceresponse).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct AddInvoiceResponse {
    pub r_hash: Base64String,
    pub payment_request: String,
    pub add_index: String,
    pub payment_addr: Base64String,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/lookup-invoice#lnrpcinvoice).
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct LookupInvoiceResponse {
    pub memo: String,
    pub r_preimage: Base64String,
    pub r_hash: Base64String,
    pub value: String,
    pub value_msat: String,
    pub settled: bool,
    pub settle_date: String,
    pub creation_date: String,
    pub payment_request: String,
    pub expiry: String,
    pub cltv_expiry: String,
    pub private: bool,
    pub add_index: String,
    pub settle_index: String,
    pub amt_paid: String,
    pub amt_paid_sat: String,
    pub amt_paid_msat: String,
    pub state: InvoiceState,
    pub status: Option<u32>, 
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcfeelimit).
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct FeeLimit {
    pub fixed: String,
    pub fixed_msat: String,
    pub percent: String,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/router/send-payment-v2/index.html).
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct SendPaymentV2Request {
    pub dest: Option<Base64String>,
    pub amt: Option<String>, 
    pub amt_msat: Option<String>,
    pub payment_hash: Option<Base64String>,
    pub final_cltv_delta: Option<i32>, 
    pub payment_addr: Option<Base64String>,
    pub payment_request: String,
    pub timeout_seconds:  Option<i32>, 
    pub fee_limit_sat: Option<String>,  
    pub fee_limit_msat: Option<String>, 
    pub outgoing_chan_id: Option<String>, 
    pub outgoing_chan_ids:  Option<Vec<String>>, //<array>
    pub last_hop_pubkey: Option<Base64String>,
    pub cltv_limit:  Option<i32>, 
    pub route_hints: Option<RouteHint>, //<array>
    pub dest_custom_records: Option<DestCustomRecordsEntry>, //<object>
    pub allow_self_payment: Option<bool>, 
    pub dest_features: Option<FeatureBit>, //<array> ENUM
    pub max_parts: Option<i64>,
    pub no_inflight_updates: Option<bool>,
    pub max_shard_size_msat: Option<String>, 
    pub amp: Option<bool>,
    pub time_pref:  Option<f64>, //<double>  
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/router/send-payment-v2/index.html#lnrpcroutehint).
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct RouteHint {
    pub hop_hints: Vec<String>, //<array>
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync/index.html#lnrpcsendrequestdestcustomrecordsentry).
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct DestCustomRecordsEntry {
    pub key: u64,
    pub value: Vec<u8>,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcmpprecord).
#[derive(Debug,  Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct MppRecord {
    pub payment_addr: Base64String,
    pub total_amt_msat: String,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcamprecord).
#[derive(Debug,  Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct AmpRecord {
    pub root_share: Base64String,
    pub set_id: Base64String,
    pub child_index: i64,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpchop).
#[derive(Debug,  Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Hop {
    pub chan_id: String,
    pub chan_capacity: String,
    pub amt_to_forward: String,
    pub fee: String,
    pub expiry: i64,
    pub amt_to_forward_msat: String,
    pub fee_msat: String,
    pub pub_key: Option<String>,
    pub tlv_payload: bool,
    pub mpp_record: Option<MppRecord>,
    pub amp_record: Option<AmpRecord>,
    pub custom_records: HashMap<String, String>,
    pub metadata: Base64String,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcroute).
#[derive(Debug,  Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Route {
    pub total_time_lock: i64,
    pub total_amt: String,
    pub total_amt_msat: String,
    pub total_fees: String,
    pub total_fees_msat: String,
    pub hops: Vec<Hop>,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/router/send-payment-v2/index.html#lnrpcpayment).
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct SendPaymentV2Response {
    pub payment_hash: String,
    pub value: String, //Deprecated
    pub creation_date: String, //Deprecated
    pub fee: String,//Deprecated
    pub payment_preimage: String,
    pub value_sat: String,
    pub value_msat: String,
    pub payment_request: String,
    pub status: PaymentStatus,
    pub fee_sat: String,
    pub fee_msat: String,
    pub creation_time_ns: String,
    pub htlcs: Vec<HTLCAttempt>, //<array>
    pub payment_index: String,
    pub failure_reason: PaymentFailureReason
}

// Final JSON struct define to myself
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct CustomSendPaymentV2Response {
    pub payment_hash: String,
    pub payment_preimage: String,
    pub payment_index: String,
    pub value_sat: String,
    pub fee: String,
    pub creation_date: String,
    pub status: PaymentStatus,
    pub num_hops: Option<u32>,
    pub code_payment_status: Option<u32>,
    pub code_failure_status: Option<u32>,
    pub status_message: Option<String>,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments#lnrpclistpaymentsrequest).
#[derive(Debug, Default, Serialize)]
pub struct ListPaymentsRequest {
    pub include_incomplete: bool,
    pub index_offset: u64,
    pub max_payments: u64,
    pub reversed: bool,
    pub count_total_payments: bool,
    pub creation_date_start: u64,
    pub creation_date_end: u64,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments/index.html#lnrpcchannelupdate).
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct ChannelUpdate {
    pub signature: String,
    pub chain_hash: String,
    pub chan_id: String,
    pub timestamp: u32,
    pub message_flags: u32,
    pub channel_flags: u32,
    pub time_lock_delta: u32,
    pub htlc_minimum_msat: String,
    pub base_fee: u32,
    pub fee_rate: u32,
    pub htlc_maximum_msat: String,
    pub extra_opaque_data: String,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments/index.html#lnrpcfailure).
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Failure {
    pub code: FailureCode,
    pub channel_update: Option<ChannelUpdate>,
    pub htlc_msat: String,
    pub onion_sha_256: String,
    pub cltv_expiry: u32,
    pub flags: u32,
    pub failure_source_index: u32,
    pub height: u32,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments/index.html#lnrpchtlcattempt).
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct HTLCAttempt {
    pub attempt_id: String,
    pub status: HTLCStatus,
    pub route: Route,
    pub attempt_time_ns: String,
    pub resolve_time_ns: String,
    pub failure: Option<Failure>,
    pub preimage: String,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments/index.html#lnrpcpayment).
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Payment {
    pub payment_hash: Base64String,
    pub payment_preimage: Base64String,
    pub payment_request: String,
    pub status: PaymentStatus,
    pub fee_sat: String,
    pub fee_msat: String,
    pub value_sat: String,
    pub value_msat: String,
    pub creation_time_ns: String,
    pub htlcs: Vec<HTLCAttempt>,
    pub payment_index: String,
    pub failure_reason: PaymentFailureReason,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments#lnrpclistpaymentsresponse).
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct ListPaymentsResponse {
    pub payments: Vec<Payment>,
    pub first_index_offset: String,
    pub last_index_offset: String,
    pub total_num_payments: String,
}

/// See [Lightning documentation BOLT11] https://github.com/lightning/bolts/blob/master/11-payment-encoding.md#encoding-overview
#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct DecodeInvoice {
    pub amount: u64,
    pub payment_hash: String,
}

// Define una función para obtener el indicador numérico del estado
pub fn get_state_indicator(state: &InvoiceState) -> i32 {
    match state {
        InvoiceState::OPEN => 0,
        InvoiceState::SETTLED => 1,
        InvoiceState::CANCELED => 2,
        InvoiceState::ACCEPTED => 3,
        InvoiceState::EXPIRED => 4,
    }
}

pub fn get_payment_status(state: &PaymentStatus) -> i32 {
    match state {
        PaymentStatus::UNKNOWN => 0,
        PaymentStatus::IN_FLIGHT => 1,
        PaymentStatus::SUCCEEDED => 2,
        PaymentStatus::FAILED => 3,
    }
}

pub fn get_payment_failure_status(state: &PaymentFailureReason) -> i32 {
    match state {
        PaymentFailureReason::FAILURE_REASON_NONE => 0,
        PaymentFailureReason::FAILURE_REASON_TIMEOUT => 1,
        PaymentFailureReason::FAILURE_REASON_NO_ROUTE => 2,
        PaymentFailureReason::FAILURE_REASON_ERROR => 3,
        PaymentFailureReason::FAILURE_REASON_INCORRECT_PAYMENT_DETAILS => 4,
        PaymentFailureReason::FAILURE_REASON_INSUFFICIENT_BALANCE => 5,
    }
}