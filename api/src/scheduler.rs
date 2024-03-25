use infrastructure as db;
use application::{lightning,lnurl::resolv_ln_address};
use diesel::prelude::*;

use domain::models::{Invoice, PaymentSplit};
use domain::modelsext::InvoiceFilters;


use chrono::{Duration, Utc};
use tracing::info;

pub async fn start_scheduler() {
    info!("Creating scheduler");

    let exp_seconds = 60;

    //job1_change_invoice_status_to_paid(exp_seconds).await;
    job2_change_invoice_status_to_distributed(exp_seconds).await;

    info!("Scheduler Started");
}

async fn job1_change_invoice_status_to_paid(exp_seconds : i32) {
    info!("Job 1 - Changing state of invoices to paid");

    tokio::spawn(async move {  

        loop {
            info!("Job 1 - Checking list of invoice");        

            use domain::schema::invoices::dsl::*;
         
            let mut conn = db::connection().expect("database connection");

            let invoice_list = invoices.filter(status.eq("0").or(status.eq("3"))).load::<Invoice>(&mut conn);

            match invoice_list {
                Ok(rows) => {
                    for row in rows {
                        info!("Job 1 - PresellId: {} / OrderId: {} / SplitId: {} / Hash: {:?}", row.presell_id, row.order_id, row.split_id, row.payment_hash);        
                        let filter = InvoiceFilters {
                            hash: row.payment_hash.expect("There's no hash")
                        };
        
                        match lightning::LndConnector::get_invoice(filter).await {
                            Ok(re) => {
                                if re.paid {
                                    diesel::update(invoices.filter(payment_hash.eq(&re.hash)))
                                    .set((payment_secret.eq(&re.preimage), status.eq(re.status)))
                                    .execute(&mut conn)
                                    .expect("Error updating invoice");
                                } else {
                                    diesel::update(invoices.filter(payment_hash.eq(&re.hash)))
                                    .set(status.eq(re.status))
                                    .execute(&mut conn)
                                    .expect("Error updating invoice");
                                }
                                info!("Job1 - Updating invoice {:?}", re.paid)
                            },
                            Err(err) => eprintln!("Job1 - Error: {:?}", err)
                        }

                    }
                }
                Err(err) => eprintln!("Job1 - Error: {:?}", err),
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
                    
        }
    });

}        

async fn job2_change_invoice_status_to_distributed(exp_seconds : i32) {
    info!("Job 2 - Changing state of invoices to distributed");

    tokio::spawn(async move {  

        loop {
            info!("Job 2 - Checking list of invoice");        

            use domain::schema::invoices;
            use domain::schema::payment_split;
            use domain::schema::invoices::dsl::*;
            use domain::schema::payment_split::dsl::*;
         
            let mut conn = db::connection().expect("database connection");

            let invoice_list = invoices.filter(invoices::status.eq("1").and(invoices::apply_split.eq(true)).and(invoices::distributed.eq(false))).load::<Invoice>(&mut conn);

            match invoice_list {
                Ok(rows) => {
                    for row in rows {
                        let mut error = 0;
                        let split_list = payment_split.filter(payment_split::invoice_id.eq(row.id).and(payment_split::status.eq("0"))).load::<PaymentSplit>(&mut conn);
                        
                        match split_list {
                            Ok(rows2) => {
                                for add in rows2 {
                                    info!("Job 2 - InvoiceId: {} / LnAddress: {} / Satoshis: {}", row.id, add.lnaddress, add.amount_msat);        
                                    match resolv_ln_address(&add.lnaddress, add.amount_msat as u64).await {
                                        Ok(ln) => {
                                            match lightning::LndConnector::send_payment(&ln, add.amount_msat as i64).await { 
                                                Ok(true) => {
                                                    diesel::update(payment_split.filter(payment_split::id.eq(add.id)))
                                                    .set((payment_split::bolt11.eq(ln), payment_split::status.eq("1"), attempts.eq(attempts + 1)))
                                                    .execute(&mut conn)
                                                    .expect("Error updating lnaddress");
                                                },
                                                Ok(false) => {
                                                    error = error + 1;
                                                    diesel::update(payment_split.filter(payment_split::id.eq(add.id)))
                                                    .set(attempts.eq(attempts + 1))
                                                    .execute(&mut conn)
                                                    .expect("Error updating lnaddress");
                                                }     
                                                Err(err) => {
                                                    error = error + 1;
                                                    eprintln!("Job2 - Error: {:?}", err)
                                                }    
                                            }    
                                        },
                                        Err(err) => {
                                            error = error + 1;
                                            eprintln!("Job2 - Error: {:?}", err)
                                        }  
                                    } 
                                }   
                            },
                            Err(err) => {
                                error = error + 1;
                                eprintln!("Job2 - Error: {:?}", err)
                            }                                          
                        }

                        if error == 0 {
                            diesel::update(invoices.filter(invoices::id.eq(row.id)))
                            .set(distributed.eq(true))
                            .execute(&mut conn)
                            .expect("Error updating distributed");
                        }
    
                    }

                }
                Err(err) => eprintln!("Job2 - Error: {:?}", err),
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
                    
        }
    });

}        