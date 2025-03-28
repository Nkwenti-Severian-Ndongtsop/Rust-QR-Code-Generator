use crate::qr_generator::generate_qr;
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

use crate::user_input::user_input;
#[derive(Deserialize)]
pub struct QRRequest {
    data: String,
    size: Option<u32>,
    format: String,
}

pub async fn generate_qr_handler() -> impl IntoResponse {

    let (d,s,f) = user_input();
    let parameters = QRRequest { data: d, size: s, format: f };

    let size = parameters.size.unwrap_or(300);
    let format = parameters.format;
    let qr_image = generate_qr(parameters.data.as_str(), size, format.as_str());

    println!("Thanks for using QR-Code: by @Nkwenti@severian\n");

    qr_image
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/generate-qr", post(generate_qr_handler))
        .route("/", get(root))
}

pub async fn root() -> &'static str {
    "Hello Qr_code Running"
}
