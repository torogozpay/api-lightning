// shared/src/response_models.rs

use domain::models::Invoice;
use domain::modelsext::InvoiceFilters;
use serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Invoice(Invoice),
    InvoiceFilters(InvoiceFilters),
}

#[derive(Serialize)]
pub struct Response {
    pub body: ResponseBody,
}