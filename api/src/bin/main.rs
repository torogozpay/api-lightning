use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;

use infrastructure as db;

use api::{invoice_handler, pay_handler, test_handler, swagger};
use api::scheduler::start_scheduler;
use shared::settings::CONFIG;


use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../infrastructure/migrations");

use tracing_appender::rolling::{Rotation, RollingFileAppender};
use tracing::info;
use std::env;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/lightning/v1")
        .service(test_handler::get_test_handler)
        .service(invoice_handler::create_invoice_handler)
        .service(invoice_handler::get_invoice_handler)
        .service(invoice_handler::get_order_handler)
        .service(invoice_handler::check_invoice_handler)
        //.service(invoice_handler::lookup_payment_handler)
        .service(pay_handler::get_verify_address_handler)
        .service(pay_handler::get_payment_handler)
    );
}

fn set_routes(config: &mut web::ServiceConfig) {
    let cnf = CONFIG.openapi.swagger.clone();
    if cnf {
        swagger::init_swagger(config);
    }
    init_routes(config);
}


type DB=diesel::pg::Pg;
fn run_migrations(connection: &mut impl MigrationHarness<DB>) {
    let _=connection.run_pending_migrations(MIGRATIONS);
} 



#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tracing using RUST_LOG
    env::set_var("RUST_LOG", CONFIG.log.level.clone());

     // Configuring tracing appender
     let file_appender = RollingFileAppender::builder()
     .rotation(Rotation::DAILY) // rotate log files once per day
     .filename_prefix("API_Lightning.logging") // log files will have names like "xxx.logging.2024-01-09"
     .build("./logs") // write log files to the '/logs' directory
     .expect("failed to initialize rolling file appender");
     
     let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
 
     tracing_subscriber::fmt()
         .with_writer(non_blocking)
         .init();
 
    // Configuring migrations
    db::init();
    let mut conn = db::connection().expect("database connection");
    run_migrations(&mut conn);

    // Configuring server
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || App::new()  
    .configure(set_routes));

    server = match listenfd.take_tcp_listener(0) { 
        Ok(Some(listener)) => server.listen(listener).expect("There is no listener"), 
        Ok(None) => {
            let host = CONFIG.server.host.clone();
            let port = CONFIG.server.port.clone();
            server.bind(format!("{host}:{port}")).expect("There is no  host and port")  
        },
        Err(_) => panic!("Unable to start API Lightning")
    };

    start_scheduler().await;

    info!("🚀 API Lightning started successfully");
    
    let _ = server.run().await;

    Ok(())
}
