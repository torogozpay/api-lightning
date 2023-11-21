use anyhow::anyhow;
use easy_hasher::easy_hasher::*;
use rand::{self, RngCore};
use uuid::Uuid;
use cln_rpc::{ClnRpc, Request, Response, model::requests::GetinfoRequest, model::requests::InvoiceRequest, model::requests::ListinvoicesRequest};
use cln_rpc::primitives::*;

use std::path::PathBuf;
use domain::models::{Invoice as MyInvoice,InvoiceFilters};


pub struct ClnConnector {
  pub sock: PathBuf
}

impl ClnConnector {
    pub async fn new(path : String) -> Self {
        let sock = path.into();
        Self { sock }
    }

    pub async fn getinfo(&mut self) -> Result<Response, anyhow::Error> {
        let mut rpc = ClnRpc::new(self.sock.clone()).await?;
        let response = rpc
            .call(Request::Getinfo(GetinfoRequest {}))
            .await
            .map_err(|e| anyhow!("Error calling getinfo: {:?}", e))?;
        
        println!("{}", serde_json::to_string_pretty(&response)?);
    
        Ok(response)
    }
    
    pub async fn create_invoice(&mut self,invoice: MyInvoice) -> Result<Response, anyhow::Error> {
        let mut rpc = ClnRpc::new(self.sock.clone()).await?;

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
    
        Ok(response)
    }
    
    pub async fn get_invoice(&mut self,invoice_filters: InvoiceFilters) -> Result<Response, anyhow::Error> {
        let mut rpc = ClnRpc::new(self.sock.clone()).await?;
    
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
    
        Ok(response)
    }
    
}