use application::lightning::types::{InvoiceState, PaymentStatus};
use application::lightning::types::{get_state_indicator, get_payment_status};
use application::lightning::invoice::is_expired_invoice;
use application::{lightning::LndConnector, lnurl::resolv_ln_address, lnreport::api_business_auth, lnreport::process_order};
use application::invoice::update::update_status_in_invoice;
use infrastructure as db;
use diesel::prelude::*;

use domain::models::{Invoice, PaymentSplit};
use domain::modelsext::InvoiceFilters;

use shared::settings::CONFIG; 
use anyhow::Result;
use chrono::{Duration, Utc};
use tracing::{info, error};

use domain::schema::invoices;
use domain::schema::payment_split;
use domain::schema::invoices::dsl::*;
use domain::schema::payment_split::dsl::*;

pub async fn start_scheduler() {
    info!("Creating scheduler");

    let exp_seconds = CONFIG.job.seconds.clone();

    job1_change_invoice_status_to_paid(exp_seconds).await;
    job2_change_invoice_status_to_distributed(exp_seconds*2).await;
    job3_change_invoice_status_to_processed(exp_seconds).await;

    info!("Scheduler Started");
}

async fn job1_change_invoice_status_to_paid(exp_seconds : i32) {
    info!("Job 1 - Changing state of invoices to paid");

    tokio::spawn(async move {  

        loop {
            info!("Job 1 - Checking list of invoice");        
            let mut conn = db::connection().expect("database connection");

            use domain::schema::invoices::dsl::*;

            let invoice_list = invoices.filter(status.eq(get_state_indicator(&InvoiceState::OPEN)).or(status.eq(get_state_indicator(&InvoiceState::ACCEPTED)))).load::<Invoice>(&mut conn);

            match invoice_list {
                Ok(rows) => {
                    for row in rows {
                        info!("Job 1 - Id: {} / PresellId: {} / OrderId: {} / Hash: {:?}", row.id, row.presell_id, row.order_id, row.payment_hash);        
                        let filter = InvoiceFilters {
                            hash: row.payment_hash.expect("There's no hash")
                        };
        
                        let expired: bool;

                        match is_expired_invoice(&row.bolt11) {
                            Ok(exp) => {
                                expired = exp;
                            },  
                            Err(_) => {
                                info!("Job1 - Error: Invoice has not been located in node" );
                                expired = true;
                            }    
                        }

                        if expired {
                            info!("Job1 - Invoice Expired = {:?}", row.id);
                            diesel::update(invoices.filter(id.eq(row.id)))
                            .set(status.eq(get_state_indicator(&InvoiceState::EXPIRED)))
                            .execute(&mut conn)
                            .expect("Error updating invoice");
                        }

                        match update_status_in_invoice(filter).await {
                            Ok(paid) => {
                                if paid {
                                    info!("Job1 - Invoice Paid = {:?}", row.id)
                                }
                            },
                            Err(err) => {
                                error!("Job1 - Error: {:?}", err )
                            }    
                        }                    
        
                    }
                }
                Err(err) => error!("Job1 - Error: {:?}", err),
            }


            let now = Utc::now();
            let next_tick = now
                .checked_add_signed(Duration::seconds(exp_seconds as i64))
                .unwrap();
            info!(
                "Job 1 - Next tick for late action users check is {}",
                next_tick.format("%a %b %e %T %Y")
            );

            tokio::time::sleep(tokio::time::Duration::from_secs(exp_seconds as u64)).await;
            drop(conn);
        }

    });

}        

async fn job2_change_invoice_status_to_distributed(exp_seconds : i32) {
    info!("Job 2 - Changing state of invoices to distributed");

    tokio::spawn(async move {  

        loop {
            info!("Job 2 - Checking list of invoice");        
            let mut conn = db::connection().expect("database connection");

            let invoice_list = invoices.filter(invoices::status.eq(get_state_indicator(&InvoiceState::SETTLED)).and(invoices::distributed.eq(false))).load::<Invoice>(&mut conn);

            match invoice_list {
                Ok(rows) => {
                    for row in rows {
                        let mut error : i32 = 0;
                        let split_list = payment_split.filter(payment_split::invoice_id.eq(row.id).and(payment_split::status.eq(get_payment_status(&PaymentStatus::UNKNOWN)).or(payment_split::status.eq(get_payment_status(&PaymentStatus::FAILED))))).load::<PaymentSplit>(&mut conn);
                        match split_list {
                            Ok(rows2) => {
                                for add in rows2 {
                                    match job2_assistant(add.id, add.invoice_id, add.bolt11, add.lnaddress, add.amount_sat.into(), row.description.clone()).await {
                                        Ok(resp) => {
                                            error = error + resp;
                                        },
                                        Err(_) => { 
                                            error = error + 1;
                                        }        
                                    }    
                                }

                                if error == 0 {
                                    diesel::update(invoices.filter(invoices::id.eq(row.id)))
                                    .set(distributed.eq(true))
                                    .execute(&mut conn)
                                    .expect("Error updating distributed");
                                }

                            },
                            Err(err) => {
                                error!("Job2 - Error split_list: {:?}", err)
                            }                                
                        }
    
                    }

                }
                Err(err) => error!("Job2 - Error invoice_list: {:?}", err),
            }


            let now = Utc::now();
            let next_tick = now
                .checked_add_signed(Duration::seconds(exp_seconds as i64))
                .unwrap();
            info!(
                "Job 2 - Next tick for late action users check is {}",
                next_tick.format("%a %b %e %T %Y")
            );

            tokio::time::sleep(tokio::time::Duration::from_secs(exp_seconds as u64)).await;
            drop(conn);
                    
        }

    });

}        

async fn job2_assistant(lid: i32, linvoice_id: i32, lbolt11: Option<String>, llnaddress: String, lamount_sat: i64, ldescription: String) -> Result<i32, anyhow::Error> {
    let mut error: i32 = 0;
    let generate: bool;


    info!("Job 2 - Id: {} / InvoiceId: {} / LnAddress: {} / Satoshis: {}", lid, linvoice_id, llnaddress, lamount_sat);        

    let mut conn = db::connection().expect("database connection");
    let rpc = LndConnector::new().await;

    match lbolt11 {
        Some(ref nextinvoice) => {
            match is_expired_invoice(&nextinvoice) {
                Ok(check) => {
                    generate = check.clone();
                    info!("Expired={:?}", check.clone())
                }
                Err(_) => {
                    generate = true;
                    info!("Expired=error decode")
                } 
            }
        },
        None => {
            generate = true;
        }
    }

    info!("Generate New Invoice={:?}", generate.clone());
    let mut myinvoice: Option<String> = Some("".to_string());

    if generate {
        match resolv_ln_address(&llnaddress, lamount_sat as u64, &ldescription).await {
            Ok(ln) => {
                if ln.clone().len() == 0 {
                    error = error + 1;
                    info!("LnAddress invalid. This payment won't be sent")
                } else {
                    myinvoice = Some(ln.clone());
                    info!("Create Bolt11={:?}",ln)
                }
            }    
            Err(err) => {
                error = error + 1;
                error!("Job2 - Error lnAddress: {:?}", err)
            } 
        } 
    } else {
        myinvoice = Some(lbolt11.clone().unwrap_or("".to_string()));
        info!("Update Bolt11={:?}", lbolt11.clone())
    }


    if error == 0 {
        match myinvoice {
            Some(nextinvoice) => {
                match rpc?.send_payment(&nextinvoice, ldescription, lamount_sat as i64).await { 
                    Ok(pay) => {
                        diesel::update(payment_split.filter(payment_split::id.eq(lid)))
                        .set((payment_split::bolt11.eq(pay.payment_request), 
                                payment_split::fee_sat.eq(pay.fee_sat.parse::<i32>().unwrap()), 
                                payment_split::status.eq(get_payment_status(&pay.status)), 
                                payment_split::payment_hash.eq(pay.payment_hash),
                                payment_split::payment_secret.eq(pay.payment_preimage),
                                attempts.eq(attempts + 1)))
                        .execute(&mut conn)
                        .expect("Job2 - Error updating lnAddress");
                        
                        if pay.status == PaymentStatus::FAILED {
                            error = error + 1;
                        }
    
                    },
                    Err(_) => {
                        error = error + 1;
                        diesel::update(payment_split.filter(payment_split::id.eq(lid)))
                        .set(attempts.eq(attempts + 1))
                        .execute(&mut conn)
                        .expect("Job2 - Error paying lnAddress");
                    }    
                }    
                 
            },
            None => {
                info!("Job2 - There's no invoice");
                error = error + 1;            
            }   
        }
    }

    drop(conn);

    Ok(error)      
}

 
async fn job3_change_invoice_status_to_processed(exp_seconds : i32) {
    info!("Job 3 - Changing state of invoices to processed");

    tokio::spawn(async move {  

        loop {
            info!("Job 3 - Checking list of invoice");        
            let mut conn = db::connection().expect("database connection");

            match api_business_auth().await { 
                Ok(jwt) => {
                    let invoice_list = invoices.filter(invoices::status.eq(get_state_indicator(&InvoiceState::SETTLED)).and(invoices::apply_split.eq(true))).load::<Invoice>(&mut conn);

                    match invoice_list {
                        Ok(rows) => {
                            for row in rows {
                                let split_list = payment_split.filter(payment_split::invoice_id.eq(row.id).and(payment_split::status.eq(get_payment_status(&PaymentStatus::SUCCEEDED))).and(payment_split::reported.eq(false))).load::<PaymentSplit>(&mut conn);
                                
                                match split_list {
                                    Ok(rows2) => {
                                        for add in rows2 {
                                            info!("Job 3 - Id: {} / InvoiceId: {} / InvoiceUid: {}/ LnAddress: {} / Satoshis: {}", add.id, add.invoice_id, row.presell_id, add.lnaddress, add.amount_sat);        
                                            match process_order(jwt.clone(), row.presell_id, add.lnaddress).await {
                                                Ok(_ln) => {
                                                    if _ln.success {
                                                        diesel::update(payment_split.filter(payment_split::id.eq(add.id)))
                                                        .set(reported.eq(true))
                                                        .execute(&mut conn)
                                                        .expect("Error updating distributed");
                                                    } else {
                                                        error!("Job3- Error: {:?}", _ln.message)
                                                    }    
                                                }    
                                                Err(err) => {
                                                    error!("Job3- Error: {:?}", err)
                                                }  
                                            } 
                                        }   
                                    },
                                    Err(err) => {
                                        info!("Job3 - Error: {:?}", err)
                                    }                                          
                                }
            
                            }
    
                        }
                        Err(err) => error!("Job3 - Error: {:?}", err),
                    }
                       
                }
                Err(_) => {
                    error!("Job3 - Error Token: YES");
                }  
            }

            let now = Utc::now();
            let next_tick = now
                .checked_add_signed(Duration::seconds(exp_seconds as i64))
                .unwrap();
            info!(
                "Job 3 - Next tick for late action users check is {}",
                next_tick.format("%a %b %e %T %Y")
            );

            tokio::time::sleep(tokio::time::Duration::from_secs(exp_seconds as u64)).await;
            drop(conn);
                    
        }

    });

}