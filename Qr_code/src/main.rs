use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use user_input::user_input;

mod auth;
mod db;
mod user_input;

#[derive(Serialize, Deserialize)]
pub struct QRRequest {
    data: String,
    size: Option<u32>,
    format: String,
}

#[tokio::main]
async fn main() {
    let url = "http://121.0.0.1:7878/generate-qr";
    let (f, s, d) = user_input();

    let request_input = QRRequest {
        data: d,
        size: s,
        format: f,
    };

    let client = Client::new();
    match client.post(url).json(&request_input).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let body = match response
                    .text()
                    .await {
                        Ok(res) => {
                            res
                        },
                        Err(e) => {
                            eprint!("Error generating QR-Code: {}", e);
                            return
                        },
                    };
                    
                println!("QR Code generated successfully: {}", body);
            } else {
                eprintln!("Failed to generate QR Code. Status: {}", response.status());
            }
        }
        Err(e) => {
            eprintln!("Error occurred while sending request: {}", e);
        }
    }
}
