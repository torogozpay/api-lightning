use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use domain::models::Invoice;
use crate::invoice_handler as invoice;

#[derive(OpenApi)]
#[openapi(
    paths(
          invoice::create_invoice_handler,
          invoice::get_invoice_handler,
          invoice::get_info_handler
    ),
    components(schemas(Invoice))
)]
pub struct ApiDoc;

pub fn init_swagger(config: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();
    config.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi));
}