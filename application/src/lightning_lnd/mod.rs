use anyhow::{anyhow,Result};
use easy_hasher::easy_hasher::*;

use nostr_sdk::nostr::secp256k1::rand::{self, RngCore};
use lnd_grpc_rust::invoicesrpc::AddHoldInvoiceRequest;
use lnd_grpc_rust::lnrpc::{PaymentHash, invoice::InvoiceState, GetInfoRequest};
use lnd_grpc_rust::{LndClient, LndClientError};

use domain::models::{Invoice as MyInvoice,InvoiceFilters,InfoNode};
use domain::modelsext::{InvoiceResponse as MyInvoiceResponse,InfoResponse};


pub struct LndConnector {
    pub client: LndClient,
}


impl LndConnector {
    pub async fn new(socket: String, cert: String, macaroon: String) -> Self {
        let client = lnd_grpc_rust::connect(
            cert,
            macaroon,
            socket,
        )
        .await
        .expect("Failed connecting to LND");

        Self { client }
    }

    pub async fn getinfo(data: InfoNode) -> Result<InfoResponse, anyhow::Error> {
        let mut rpc = LndConnector::new(data.socket.clone(), data.cert.clone(), data.macaroon.clone()).await;
        
        let response = rpc
            .client
            .lightning()
            .get_info(GetInfoRequest {})
            .await
            .map_err(|e| anyhow!("Error calling getinfo: {:?}", e))?
            .into_inner();
        
        println!("{:?}", response);
    
        let id = response.identity_pubkey.to_string();
        let alias = Some(response.alias.to_string());
        let block = response.block_height.to_string();

        Ok(InfoResponse {
            identity_pubkey: id,
            alias: alias,
            block_height: block
        })
    }
    

    pub async fn create_invoice(invoice: MyInvoice) -> Result<MyInvoiceResponse, LndClientError> {
        let mut rpc = LndConnector::new(invoice.socket.clone(), invoice.cert.clone(), invoice.macaroon.clone()).await;

        let mut preimage = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut preimage);
        let hash = raw_sha256(preimage.to_vec());

        let newinvoice = AddHoldInvoiceRequest {
            hash: hash.to_vec(),
            memo: invoice.description.to_string(),
            value: invoice.amount,
            cltv_expiry: invoice.cltv as u64,
            expiry: invoice.expiry as i64,
            ..Default::default()
        };

        let holdinvoice = rpc
            .client
            .invoices()
            .add_hold_invoice(newinvoice)
            .await
            .expect("Failed to add hold invoice")
            .into_inner();

        println!("{:?}", holdinvoice.clone());

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
        let warnings = "".to_string();

        Ok(MyInvoiceResponse {
            payment_request: Some(bolt11),
            preimage: Some(preimage),
            hash: Some(hash),
            paid: false,
            warnings: Some(warnings),            
            ..Default::default()
        })
    }

    pub async fn get_invoice(invoice_filters: InvoiceFilters) -> Result<MyInvoiceResponse, LndClientError> {
        let mut rpc = LndConnector::new(invoice_filters.socket.clone(), invoice_filters.cert.clone(), invoice_filters.macaroon.clone()).await;
        
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

        println!("{:?}", invoice);
            
        let resp : MyInvoiceResponse;    
        if Some(invoice.clone()) != None {
            let bolt11 = invoice.payment_request;
            let warnings = "".to_string();

            let mut preimage = String::new(); 
            let mut hash = String::new(); 
            let mut paid = false;
            let mut expires = Some(0);

            if let Some(state) = InvoiceState::from_i32(invoice.state) {
                if state == InvoiceState::Settled {
                    paid = true;
                    preimage = invoice
                        .r_preimage
                        .iter()
                        .map(|h| format!("{h:02x}"))
                        .collect::<Vec<String>>()
                        .join("");
                    hash = invoice
                        .r_hash
                        .iter()
                        .map(|h| format!("{h:02x}"))
                        .collect::<Vec<String>>()
                        .join("");     
                    expires = Some(invoice.settle_date.clone());               
                }
            }

            resp = MyInvoiceResponse {
                payment_request: Some(bolt11),
                preimage: Some(preimage),
                hash: Some(hash),        
                paid: paid,
                expires_at: expires,
                warnings: Some(warnings),
                ..Default::default()
            }
        } else {   
            resp = MyInvoiceResponse {
                warnings: Some("Invoice not found".to_string()),            
                ..Default::default()
            }            
        }

        Ok(resp)
    }     
}
