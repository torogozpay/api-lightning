use anyhow::anyhow;
use cln_rpc::{ClnRpc, Request, Response, model::requests::GetinfoRequest, model::requests::InvoiceRequest, model::requests::ListinvoicesRequest};
use std::path::PathBuf;

pub struct ClnConnector {
  pub sock: PathBuf
}

impl ClnConnector {
    pub async fn new() -> Self {
        let sock = "/data/lightningd/bitcoin/lightning-rpc".into();
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
    
    pub async fn create_invoice(&mut self,inv: InvoiceRequest) -> Result<Response, anyhow::Error> {
        let mut rpc = ClnRpc::new(self.sock.clone()).await?;
    
        let response = rpc
           .call(Request::Invoice(inv))
           .await
           .map_err(|e| anyhow!("Error calling create invoice: {:?}", e))?;
    
        println!("{}", serde_json::to_string_pretty(&response)?);
    
        Ok(response)
    }
    
    pub async fn get_invoice(&mut self,inv: ListinvoicesRequest) -> Result<Response, anyhow::Error> {
        let mut rpc = ClnRpc::new(self.sock.clone()).await?;
    
        let response = rpc
           .call(Request::ListInvoices(inv))
           .await
           .map_err(|e| anyhow!("Error calling get invoice: {:?}", e))?;
    
        println!("{}", serde_json::to_string_pretty(&response)?);
    
        Ok(response)
    }
    
}