#[warn(non_camel_case_types)]
#[warn(deprecated)]
pub mod invoice;
pub mod types;

use crate::lightning::types::{
    AddInvoiceRequest, AddInvoiceResponse, LookupInvoiceResponse, 
    SendPaymentV2Request, SendPaymentV2Response 
};

use futures_util::stream::StreamExt; // Importa el trait para usar stream
use serde_json::Value; // Importa el tipo Value para analizar JSON

use shared::{settings::CONFIG, functions, error_handler::CustomError};

use anyhow::Result;

use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::ffi::OsStr;
use std::io::Write;

use reqwest::Client;
extern crate rand;
use tracing::{info, error};

use crate::lightning::types::get_state_indicator;
use crate::lightning::types::InvoiceState;

use domain::modelsext::{InvoiceData, InvoiceFilters, InvoiceResponse, LookupInvoiceResponse as LookInvoiceResponse};


/// Encapsulate data needed to interact with a Lightning Network Daemon (LND) node.
#[derive(Clone, Debug)]
pub struct LndConnector {
    /// The host address of the LND node.
    pub host: String,
    /// The HTTP client used to communicate with the LND node.
    pub client: Client,
}

impl LndConnector {
    pub async fn new() -> Result<Self> {
        // Connecting to LND requires only host, port, cert file, and macaroon file        
        let host =  "https://".to_owned() + &CONFIG.node.host.clone() + ":" + &CONFIG.node.port.clone().to_string();
        
        let mut cert_file = File::open(&CONFIG.node.cert_file.clone())?;
        let mut cert_raw = Vec::new();
        cert_file.read_to_end(&mut cert_raw)?;
    
        let cert = reqwest::Certificate::from_pem(&cert_raw)?;
    
        //info!("cert value: {:?}", cert);
        std::io::stdout().flush().unwrap();
    
        let cmd_output = Command::new("xxd")
            .args(["-ps", "-u", "-c", "1000"])
            .arg(CONFIG.node.macaroon_file.as_ref() as &OsStr)
            .output()?;
    
        let mut macaroon = cmd_output.stdout;
        macaroon.retain(|&z| {
            ((z >= b'0' as _) && (z <= b'9' as _)) | ((z >= b'A' as _) && (z <= b'F' as _))
        });
    
        //info!("macaroon value: {:?}", macaroon);
        std::io::stdout().flush().unwrap();
            
        let mut headers = reqwest::header::HeaderMap::new();
        let mut macaroon_value = reqwest::header::HeaderValue::from_bytes(&macaroon)?;
        macaroon_value.set_sensitive(true);
        headers.insert("Grpc-Metadata-macaroon", macaroon_value.clone());

        let client = reqwest::Client::builder()
            .add_root_certificate(cert)
            .default_headers(headers)
            .build()?;
    
        Ok(LndConnector { host, client })
    }

    async fn on_response(response: reqwest::Response) -> Result<reqwest::Response> {
        let status = response.status();

        match status {
            reqwest::StatusCode::OK => Ok(response),
            _ => match response.error_for_status() {
                Ok(res) => Ok(res),
                Err(err) => Err(err.into()),
            },
        }
    }    

    pub async fn create_invoice(&mut self, invoice: InvoiceData) -> Result<InvoiceResponse, CustomError> {
        let newinvoice = AddInvoiceRequest {
            memo: Some(invoice.description.to_string()),
            value: invoice.amount_sat,
            cltv_expiry: Some(CONFIG.node.cltv_expiry.clone()),
            expiry: CONFIG.node.expiry.clone(), 
            ..Default::default()
        };

        let url = format!("{host}/v1/invoices", host = self.host);

        let mut response = match self.client.post(&url).json(&newinvoice).send().await {
            Ok(response) => response,
            Err(err) => {
                error!("Error in create_invoice: {:?}", err);
                return Err(CustomError::new(500, "Internal server error".to_string()))
            }    
        };

        response = Self::on_response(response).await?;
        let crinvoice: AddInvoiceResponse = response.json().await?;        

        info!("Invoice: {:?}", crinvoice.clone());

        let bolt11 = crinvoice.payment_request;
        let hash = functions::base64_to_hex(crinvoice.r_hash);

        let status = get_state_indicator(&InvoiceState::OPEN); 

        Ok(InvoiceResponse {
            invoice_request: bolt11,
            hash: Some(hash),
            paid: false,
            status: status,
            ..Default::default()
        })
    }


    pub async fn get_invoice(&mut self, invoice_filters: InvoiceFilters) -> Result<LookInvoiceResponse, CustomError> {
        let url = format!(
            "{host}/v1/invoice/{payment_hash}",
            host = self.host,
            payment_hash = &invoice_filters.hash.clone()
        );

        let mut response = match self.client.get(&url).send().await {
            Ok(response) => response,
            Err(err) => {
                error!("Error (get) in get_invoice: {:?}", err);
                return Err(CustomError::new(500, "Internal server error".to_string()))
            }    
        };

        info!("Response get_invoice: {:?}", response);

        response = match Self::on_response(response).await {
            Ok(response) => response,
            Err(_) => return Err(CustomError::new(500, "Internal server error".to_string())),
        };

        let invoice: LookupInvoiceResponse = match response.json().await {
            Ok(data) => data,
            Err(err) => {
                error!("Error (Result) in get_invoice: {:?}", err);
                return Err(CustomError::new(500, "Internal server error".to_string()))
            }    
        };

        info!("Invoice: {:?}", invoice);
            
        let resp : LookInvoiceResponse;    
        if Some(invoice.clone()) != None {
            let bolt11 = invoice.payment_request;
            let status = get_state_indicator(&invoice.state);
            let mut paid = false;

            if invoice.state == InvoiceState::SETTLED {
                paid = true;
            }

            resp = LookInvoiceResponse {             
                memo: invoice.memo,
                r_preimage: invoice.r_preimage,
                r_hash: invoice.r_hash,   
                value: invoice.value.parse().unwrap(),
                value_msat: invoice.value_msat.parse().unwrap(),
                settled: invoice.settled,
                settle_date: invoice.settle_date.parse().unwrap(),
                creation_date: invoice.creation_date.parse().unwrap(),
                payment_request: bolt11,
                expiry: invoice.expiry.parse().unwrap(),
                cltv_expiry: invoice.cltv_expiry.parse().unwrap(),
                private: invoice.private,
                add_index: invoice.add_index.parse().unwrap(),
                settle_index: invoice.settle_index.parse().unwrap(),
                amt_paid: invoice.amt_paid.parse().unwrap(),
                amt_paid_sat: invoice.amt_paid_sat.parse().unwrap(),
                amt_paid_msat: invoice.amt_paid_msat.parse().unwrap(),
                paid: paid,
                state: status,
                ..Default::default()
            }
        } else {   
            resp = LookInvoiceResponse {
                ..Default::default()
            }            
        }

        Ok(resp)
    } 
    
    
    pub async fn send_payment(&mut self, payment_request: &str, _description: String, amount: i64)  -> Result<SendPaymentV2Response, CustomError> {
        let invoice = invoice::decode_invoice(payment_request).unwrap();
        info!("Decode Invoice {}", invoice);

        // We need to set a max fee amount
        let mut max_fee = (amount as f64 * CONFIG.node.max_fee.clone() as f64) as i64;
        let min_fee = CONFIG.node.min_fee.clone() as i64;

        if max_fee < min_fee {
            max_fee = min_fee;
        }

        if max_fee > amount {
            max_fee = amount;
        }

        info!("MaxFee {:?} ", max_fee);

        let request = SendPaymentV2Request  {
            payment_request: payment_request.to_string(),
            timeout_seconds: Some(CONFIG.node.pathfinding_timeout.clone() as i32),
            fee_limit_sat: Some(max_fee.to_string()),    
            outgoing_chan_ids: Some(get_channels()),
            ..Default::default()
        };

        let url = format!("{host}/v2/router/send", host = self.host);

        let mut response = match self.client.post(&url).json(&request).send().await {
            Ok(response) => response,
            Err(_) => return Err(CustomError::new(500, "Internal server error".to_string())),
        };

       
        response = match Self::on_response(response).await {
            Ok(response) => response,
            Err(_) => return Err(CustomError::new(500, "Internal server error".to_string())),
        };
        
    let mut stream = response.bytes_stream();    


    // Variables to store the complete JSON
    let mut json_str = String::new();

    // Read the data from the stream and store the JSON in a string
    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                // Convert JSON bytes to a UTF-8 string
                if let Ok(str) = std::str::from_utf8(&bytes) {
                    //Deserialize the JSON fragment to a Value to find the "status" field
                    if let Ok(json_value) = serde_json::from_str::<Value>(str) {
                        // Check if the JSON fragment contains the "result" field
                        if let Some(result) = json_value.get("result") {
                            info!("result: {:?}", result);
                            // Check if the "result" object contains the "status" field with the value "SUCCEEDED" or "FAILED"
                            if let Some(status) = result.get("status").and_then(Value::as_str) {
                                info!("status: {:?}", status);

                                if status == "SUCCEEDED" || status == "FAILED" {
                                    json_str = str.to_string();
                                    break; //Get out of the loop
                                }
                            }
                        }
                    }
                } else {
                    return Err(CustomError::new(400, "Failed to parse JSON as UTF-8".to_string()));
                }
            },
            Err(_) => return Err(CustomError::new(500, "Internal server error".to_string())),
        }
    }

    // Print the entire JSON to the console
    info!("JSON response SendPaymentV2 (stream Payment): {}", json_str);

    // Deserialize the JSON fragment into the SendPaymentV2Response structure
    let result_value = serde_json::from_str::<Value>(&json_str)?;
    let data: Option<SendPaymentV2Response> = Some(match result_value.get("result") {
            Some(v) => serde_json::from_value::<SendPaymentV2Response>(v.clone())
                        .map_err(|err| CustomError::new(400, format!("Error getting JSON result: {}", err))),
            None => Err(CustomError::new(400, "Field 'result' was not found in the JSON".to_string())),
        }?);        


    // Devolver la respuesta completa
    Ok(data.expect("error"))

    }
}



fn get_channels() -> Vec<String> {
    let mut channels = vec![];
    
    channels.push(CONFIG.node.out.to_string());
    
    return channels;
}