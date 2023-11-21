#[macro_use]
extern crate lazy_static;

use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;

use api::{invoice_c_handler, invoice_lnd_handler, test_handler, swagger};
use shared::settings;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("Config can be loaded");
}


//service(web::scope("/api/v1"))
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1")
        .service(test_handler::get_test_handler)
        .service(invoice_c_handler::get_info_handler)
        .service(invoice_c_handler::create_invoice_handler)
        .service(invoice_c_handler::get_invoice_handler)
        .service(invoice_lnd_handler::create_invoice_handler)
        .service(invoice_lnd_handler::get_invoice_handler)
    );
}

fn set_routes(config: &mut web::ServiceConfig) {
    swagger::init_swagger(config);
    init_routes(config);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || App::new().configure(set_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = CONFIG.server.host.clone();
            let port = CONFIG.server.port.clone();
            server.bind(format!("{host}:{port}"))?
        }
    };

    server.run().await
}