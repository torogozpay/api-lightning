use std::env;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use listenfd::ListenFd;

use api::invoice_handler;
use api::test_handler;
use api::swagger;


//service(web::scope("/api/v1"))
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1")
        .service(test_handler::get_test_handler)
        .service(invoice_handler::get_info_handler)
        .service(invoice_handler::create_invoice_handler)
        .service(invoice_handler::get_invoice_handler)
    );
}



fn set_routes(config: &mut web::ServiceConfig) {
    swagger::init_swagger(config);
    init_routes(config);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || App::new().configure(set_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{host}:{port}"))?
        }
    };

    server.run().await
}