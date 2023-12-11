use anyhow::anyhow;
use easy_hasher::easy_hasher::*;
use rand::{self, RngCore};
use uuid::Uuid;
use cln_rpc::{ClnRpc, Response, Request, model::requests::GetinfoRequest, model::requests::InvoiceRequest, model::requests::ListinvoicesRequest};
use cln_rpc::{model::responses::ListinvoicesInvoicesStatus};
use cln_rpc::primitives::*;

use std::path::PathBuf;
use domain::models::{Invoice as MyInvoice,InvoiceFilters, InfoNode};
use domain::modelsext::{InvoiceResponse as MyInvoiceResponse,InfoResponse};


pub struct ClnConnector {
  pub sock: PathBuf
}

impl ClnConnector {
    pub async fn new(_path : String) -> Self {
        //let sock = PathBuf::from(_path);
        let sock = PathBuf::from("/root/.lightning/bitcoin/lightning-rpc");
        Self { sock }
    }

    pub async fn getinfo(data: InfoNode) -> Result<InfoResponse, anyhow::Error> {
        let mut rpc = ClnRpc::new(data.path.clone()).await?;
        let response = rpc
            .call(Request::Getinfo(GetinfoRequest {}))
            .await
            .map_err(|e| anyhow!("Error calling getinfo: {:?}", e))?;
        
        println!("{}", serde_json::to_string_pretty(&response)?);
    
        match response {
            Response::Getinfo(getinfo) => {
                let id = getinfo.id.to_string();
                let alias = getinfo.alias;
                let block = getinfo.blockheight.to_string();
        
                Ok(InfoResponse {
                    identity_pubkey: id,
                    alias: alias,
                    block_height: block
                })
            },
            _ => panic!()
        } 

    }
    
    pub async fn create_invoice(invoice: MyInvoice) -> Result<MyInvoiceResponse, anyhow::Error> {
        let mut rpc = ClnRpc::new(invoice.path.clone()).await?;

        let amount = AmountOrAny::Amount(Amount::from_msat(invoice.amount as u64));
        let label = format!("{}", Uuid::new_v4());
    
        let mut preimage = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut preimage);
        let hash = raw_sha256(preimage.to_vec()).to_hex_string();

        let response = rpc
           .call(Request::Invoice(InvoiceRequest {
                amount_msat: amount, 
                label: label,
                description: invoice.description.clone(),
                preimage: Some(hash),
                expiry: Some(invoice.expiry.into()),
                deschashonly: None,
                cltv: Some(invoice.cltv),
                fallbacks: None 
            }))
           .await
           .map_err(|e| anyhow!("Error calling create invoice: {:?}", e))?;
    
        println!("{}", serde_json::to_string_pretty(&response)?);
    
        match response {
            Response::Invoice(getinv) => {
                let bolt11 = getinv.bolt11;
                let mut warnings = "".to_string();

                let preimage = getinv.payment_secret
                    .to_vec()
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>();                
                let hash = getinv.payment_hash.to_string();
                let expires = getinv.expires_at.try_into().unwrap();

                if getinv.warning_capacity != None {
                    warnings = warnings + " / " + &getinv.warning_capacity.expect("capacity").to_string();
                }
                if getinv.warning_offline != None {
                    warnings = warnings + " / " + &getinv.warning_offline.expect("offline").to_string();
                }
                if getinv.warning_deadends != None {
                    warnings = warnings + " / " + &getinv.warning_deadends.expect("deadends").to_string();
                }
                if getinv.warning_private_unused != None {
                    warnings = warnings + " / " + &getinv.warning_private_unused.expect("private_unused").to_string();
                }
                if getinv.warning_mpp != None {
                    warnings = warnings + " / " + &getinv.warning_mpp.expect("mpp").to_string();
                }

                Ok(MyInvoiceResponse {
                    payment_request: Some(bolt11),
                    preimage: Some(preimage),
                    hash: Some(hash),
                    paid: false,
                    expires_at: Some(expires), 
                    warnings: Some(warnings),            
                    ..Default::default()
                })
            },
            _ => panic!()
        } 

    }
    
    pub async fn get_invoice(invoice_filters: InvoiceFilters) -> Result<MyInvoiceResponse, anyhow::Error> {
        let mut rpc = ClnRpc::new(invoice_filters.path.clone()).await?;

        let response = rpc
           .call(Request::ListInvoices(ListinvoicesRequest{
                index: None, 
                invstring: None,
                label: None,
                limit: None,
                offer_id: None,
                start: None,
                payment_hash: Some(invoice_filters.hash.to_string())}))
           .await
           .map_err(|e| anyhow!("Error calling get invoice: {:?}", e))?;
    
        println!("{}", serde_json::to_string_pretty(&response)?);
    
        match response {
            Response::ListInvoices(getinv) => {
                let resp : MyInvoiceResponse;
                
                if getinv.invoices.len() > 0 {
                    let inv = &getinv.invoices[0];

                    let bolt11 = inv.bolt11.clone();
                    let warnings = "".to_string();

                    let mut preimage = String::new(); 
                    let mut hash = String::new(); 
                    let mut paid = false;
                    let mut expires = 0;

                    if ListinvoicesInvoicesStatus::PAID == inv.status {
                        paid = true;
                        preimage = inv.payment_preimage
                            .expect("Error in preimage")
                            .to_vec()
                            .iter()
                            .map(|b| format!("{:02x}", b))
                            .collect::<String>();
                        hash = inv.payment_hash.to_string();
                        expires = inv.expires_at.try_into().unwrap();
                    }

                    resp = MyInvoiceResponse {
                        payment_request: bolt11,  
                        preimage: Some(preimage),
                        hash: Some(hash),
                        paid: paid,
                        expires_at: Some(expires), 
                        warnings: Some(warnings),            
                        ..Default::default()
                    }
                    
                } else{ 
                    resp = MyInvoiceResponse {
                        warnings: Some("Invoice not found".to_string()),            
                        ..Default::default()
                    }
                }
                Ok(resp)
            },
            _ => panic!()
        } 

    }
    
}