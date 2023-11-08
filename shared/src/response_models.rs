// shared/src/response_models.rs

use domain::models::Invoice;
use serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Invoice(Invoice),
}

#[derive(Serialize)]
pub struct Response {
    pub body: ResponseBody,
}