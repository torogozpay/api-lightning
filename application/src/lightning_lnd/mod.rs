use anyhow::Result;
use hex::FromHex;
use easy_hasher::easy_hasher::*;

use nostr_sdk::nostr::secp256k1::rand::{self, RngCore};
use lnd_grpc_rust::invoicesrpc::AddHoldInvoiceRequest;
use lnd_grpc_rust::lnrpc::{PaymentHash, invoice::InvoiceState};
use lnd_grpc_rust::{LndClient, LndClientError};

use domain::models::{Invoice as MyInvoice,InvoiceFilters,InvoiceResponse};


pub struct LndConnector {
    pub client: LndClient,
}


impl LndConnector {
    pub async fn new(socket: String, cert: String, macaroon: String) -> Self {
        // Connecting to LNDuse serde::{Serialize, Deserialize}; requires only host, port, cert file, and macaroon file
        let client = lnd_grpc_rust::connect(
            cert,
            macaroon,
            socket,
        )
        .await
        .expect("Failed connecting to LND");

        Self { client }
    }

    pub async fn create_invoice(&mut self, invoice: MyInvoice) -> Result<InvoiceResponse, LndClientError> {

        let mut preimage = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut preimage);
        let hash = raw_sha256(preimage.to_vec());

        let newinvoice = AddHoldInvoiceRequest {
            hash: hash.to_vec(),
            memo: invoice.description.to_string(),
            value: invoice.amount,
            cltv_expiry: invoice.cltv as u64,
            ..Default::default()
        };

        let holdinvoice = self
            .client
            .invoices()
            .add_hold_invoice(newinvoice)
            .await
            .expect("Failed to add hold invoice")
            .into_inner();

        Ok(InvoiceResponse {
            payment_request: holdinvoice.payment_request,
            preimage: preimage.to_vec(),
            hash: hash.to_vec(),
            description: invoice.description.to_string(),        
            paid: false
        })
    }

    pub async fn get_invoice(&mut self, invoice_filters: InvoiceFilters) -> Result<InvoiceResponse, LndClientError> {
        
        let hash = <[u8; 32]>::from_hex(invoice_filters.hash.clone()).expect("Decoding failed");

        let invoice = self
              .client
              .lightning()
              .lookup_invoice(PaymentHash {
                r_hash: hash.to_vec(),
                ..Default::default()
            })
            .await?
            .into_inner();
    
        let mut preimage = String::new(); //[0u8; 32];
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

        Ok(InvoiceResponse {
            paid: paid,
            preimage: preimage.into(),
            description: invoice.memo,
            ..Default::default()
        })

    }
         
    
}
