pub mod invoice;

use std::cmp::Ordering;

use shared::{settings::CONFIG,functions};

use anyhow::{anyhow,Result};
use easy_hasher::easy_hasher::*;

extern crate rand;
use crate::lightning::rand::RngCore;
use tracing::info;

use tonic_openssl_lnd::{LndClient, LndClientError};
use tonic_openssl_lnd::lnrpc::{invoice::InvoiceState, PaymentHash, GetInfoRequest, Payment};
use tonic_openssl_lnd::invoicesrpc::AddHoldInvoiceRequest;
use tonic_openssl_lnd::routerrpc::{SendPaymentRequest,TrackPaymentRequest};

use domain::modelsext::{InvoiceData, InvoiceFilters, InvoiceResponse as MyInvoiceResponse,InfoResponse};


#[derive(Debug, Clone)]
pub struct PaymentMessage {
    pub payment: Payment,
}

pub struct LndConnector {
    pub client: LndClient,
}


impl LndConnector {
    pub async fn new() -> Self {
        // Connecting to LND requires only host, port, cert file, and macaroon file        
        let client = tonic_openssl_lnd::connect(CONFIG.node.host.clone(), CONFIG.node.port.clone().into(), CONFIG.node.cert_file.clone(), CONFIG.node.macaroon_file.clone())
        .await
        .expect("Failed connecting to LND");
        
        Self { client }
    }

    pub async fn getinfo() -> Result<InfoResponse, anyhow::Error> {
        let mut rpc = LndConnector::new().await;
        
        let response = rpc
            .client
            .lightning()
            .get_info(GetInfoRequest {})
            .await
            .map_err(|e| anyhow!("Error calling getinfo: {:?}", e))?
            .into_inner();
         
        info!("{:?}", response);
        
        let id = response.identity_pubkey.to_string();
        let alias =  Some(response.alias.to_string());
        let block = response.block_height.to_string();

        Ok(InfoResponse {
            identity_pubkey: id,
            alias:  alias,
            block_height: block
        })
    }
    

    pub async fn create_invoice(invoice: InvoiceData) -> Result<MyInvoiceResponse, LndClientError> {
        let mut rpc = LndConnector::new().await;

        let mut preimage = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut preimage);
        let hash = raw_sha256(preimage.to_vec());

        let newinvoice = AddHoldInvoiceRequest {
            hash: hash.to_vec(),
            memo: invoice.description.to_string(),
            value: invoice.amount_msat,
            cltv_expiry: CONFIG.node.cltv_expiry.clone(),
            expiry: CONFIG.node.expiry.clone(), 
            ..Default::default()
        };

        let holdinvoice = rpc
            .client
            .invoices()
            .add_hold_invoice(newinvoice.clone())
            .await
            .expect("Failed to add hold invoice")
            .into_inner();

        info!("{:?}", holdinvoice.clone());

        let bolt11 = holdinvoice.payment_request;
        let preimage = preimage
            .to_vec()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        let hash = hash
            .to_vec()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        Ok(MyInvoiceResponse {
            payment_request: Some(bolt11),
            preimage: Some(preimage),
            hash: Some(hash),
            paid: false,
            ..Default::default()
        })
    }

    pub async fn get_invoice(invoice_filters: InvoiceFilters) -> Result<MyInvoiceResponse, LndClientError> {
        let mut rpc = LndConnector::new().await;
        
        let hash = hex::decode(invoice_filters.hash.clone()).expect("Decoding failed");

        let invoice = rpc
              .client
              .lightning()
              .lookup_invoice(PaymentHash {
                r_hash: hash,
                ..Default::default()
            })
            .await?
            .into_inner();

        info!("{:?}", invoice);
            
        let resp : MyInvoiceResponse;    
        if Some(invoice.clone()) != None {
            let bolt11 = invoice.payment_request;
            let hash = invoice_filters.hash.clone(); 
            let status = invoice.state;

            let mut preimage = String::new(); 
            let mut paid = false;

            if let Some(state) = InvoiceState::from_i32(invoice.state) {
                if state == InvoiceState::Settled {
                    paid = true;
                    preimage = invoice
                        .r_preimage
                        .iter()
                        .map(|h| format!("{h:02x}"))
                        .collect::<Vec<String>>()
                        .join("");
                }
            }

            resp = MyInvoiceResponse {
                payment_request: Some(bolt11),
                preimage: Some(preimage),
                hash: Some(hash),        
                paid: paid,
                status: status.to_string(),
                ..Default::default()
            }
        } else {   
            resp = MyInvoiceResponse {
                ..Default::default()
            }            
        }

        Ok(resp)
    }  
    
    pub async fn send_payment(
        payment_request: &str,
        amount: i64
    )  -> Result<bool, LndClientError> {
        let mut rpc = LndConnector::new().await;

        let invoice = invoice::decode_invoice(payment_request).unwrap();
        info!("Decode Invoice {}", invoice);

        let payment_hash = invoice.payment_hash();
        let payment_hash = payment_hash.to_vec();
        let hash = functions::bytes_to_string(&payment_hash);

        // We need to set a max fee amount
        // If the amount is small we use a different max routing fee
        let max_fee = match amount.cmp(&100) {
            Ordering::Less | Ordering::Equal => amount as f64 * 0.1,
            Ordering::Greater => amount as f64 * CONFIG.node.max_fee.clone(),
        };

        let track_payment_req = TrackPaymentRequest {
            payment_hash,
            no_inflight_updates: true,
        };

        info!("FEE = Ordering {:?} / MaxFee Less {} / MaxFee Greater {}", amount.cmp(&100), amount as f64 * 0.1, max_fee);

        let track = rpc
            .client
            .router()
            .track_payment_v2(track_payment_req)
            .await;

        // We only send the payment if it wasn't attempted before
        if track.is_ok() {
            info!("Aborting paying invoice with hash {} to buyer", hash);
            return Ok(false);
        }

        let request = SendPaymentRequest {
            payment_request: payment_request.to_string(),
            timeout_seconds: CONFIG.node.pathfinding_timeout.clone() as i32,
            fee_limit_sat: max_fee as i64,
            ..Default::default()
        };

        let mut stream = rpc
            .client
            .router()
            .send_payment_v2(request)
            .await
            .expect("Failed sending payment")
            .into_inner();

        while let Some(payment) = stream.message().await.expect("Failed paying invoice") {
            let _msg = PaymentMessage { payment };
            info!("{:?} ", _msg);

            if _msg.payment.status == 3 {
                return Ok(false);
            } 
        }

       return Ok(true);
   }    
}
