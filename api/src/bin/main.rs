use actix_web::{web, App, HttpServer, Error};
use listenfd::ListenFd;

use api::{invoice_handler, invoice_c_handler, invoice_lnd_handler, test_handler, swagger};
use shared::settings;

use api::{invoice_handler, pay_handler, test_handler, swagger};
use api::scheduler::start_scheduler;
use shared::settings::CONFIG;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use std::env;

/*
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../infrastructure/migrations");
*/

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1")
        .service(test_handler::get_test_handler)
        .service(invoice_handler::get_info_handler)
        .service(invoice_handler::create_invoice_handler)
        .service(invoice_handler::get_invoice_handler)
        .service(invoice_c_handler::get_info_handler)
        .service(invoice_c_handler::create_invoice_handler)
        .service(invoice_c_handler::get_invoice_handler)
        .service(invoice_lnd_handler::get_info_handler)
        .service(invoice_lnd_handler::create_invoice_handler)
        .service(invoice_lnd_handler::get_invoice_handler)
    );
}

fn set_routes(config: &mut web::ServiceConfig) {
    swagger::init_swagger(config);
    init_routes(config);
}

/*
type DB=diesel::pg::Pg;
fn run_migrations(connection: &mut impl MigrationHarness<DB>) {
    let _=connection.run_pending_migrations(MIGRATIONS);
} 
*/


#[actix_rt::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", CONFIG.log.level.clone());

    // Tracing using RUST_LOG
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
        
    db::init();

    //let mut conn = db::connection().expect("database connection");
    //run_migrations(&mut conn);

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || App::new()  
    .configure(set_routes));

    server = match listenfd.take_tcp_listener(0) { //?
        Ok(Some(listener)) => server.listen(listener).expect("There is no listener"), //?
        Ok(None) => {
            let host = CONFIG.server.host.clone();
            let port = CONFIG.server.port.clone();
            server.bind(format!("{host}:{port}")).expect("There are host and port")  //?
        },
        Err(_) => panic!()
    };

    //start_scheduler().await;

    println!("🚀 API Lightning started successfully");
    
    let _ = server.run().await;

    Ok(())
}
//https://rustyfullstack.com/blog/logs-con-rust
//RUST_LOG=crear_logs_con_rust cargo run
//RUST_LOG=info cargo run