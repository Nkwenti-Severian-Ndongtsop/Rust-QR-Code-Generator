use axum::{
    response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::Deserialize;

use crate::qr_generator::generate_qr;


#[derive(Deserialize)]
pub struct QRRequest {
    data: String,
    size: Option<u32>,
    format: String,
}

pub async fn generate_qr_handler(Json(payload): Json<QRRequest>) -> impl IntoResponse {

    
    let size = payload.size.unwrap_or(300);
    let format = payload.format;
    let qr_image = generate_qr(payload.data.as_str(), size, format.as_str());

    println!("Thanks for using QR-Code: by @Nkwenti@severian");

    qr_image
}



pub async fn root() -> &'static str {
    "Hello Qr_code Running"
}

pub fn create_routes() -> Router {
        Router::new()
            .route("/generate-qr", post(generate_qr_handler))
            .route("/", get(root))
    }
