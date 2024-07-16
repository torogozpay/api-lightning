use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi,Config};
use utoipa::openapi::security::{HttpAuthScheme,Http,SecurityScheme};

use domain::models::{Invoice};
use domain::modelsext::{InvoiceData,InvoiceFilters,InvoiceCheck,OrderFilters,PreorderSplit,Payment,PaymentFilters}; 
use crate::invoice_handler as invoice;
use crate::pay_handler as pay;

#[derive(OpenApi)]
#[openapi(
    paths(
          invoice::create_invoice_handler,
          invoice::get_invoice_handler,
          invoice::get_order_handler,
          invoice::check_invoice_handler,
          pay::get_verify_address_handler,
          pay::get_payment_handler
    ),
    components(schemas(InvoiceData,Invoice,InvoiceFilters,InvoiceCheck,OrderFilters,
        PreorderSplit,Payment,PaymentFilters))
)]
pub struct ApiDoc;

pub fn init_swagger(config: &mut web::ServiceConfig) {
    let mut openapi = ApiDoc::openapi();

    let components: &mut utoipa::openapi::Components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
    components.add_security_scheme(
        "bearerAuth",
        SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
    );
    
    // Assuming you have a service method for registering routes
    let swagger = SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi)
                    .config(Config::default().try_it_out_enabled(false).filter(false)); 

    config.service(swagger);
}