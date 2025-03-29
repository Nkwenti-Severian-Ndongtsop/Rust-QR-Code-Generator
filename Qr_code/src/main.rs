use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
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
    let url = "http://127.0.0.1:7878/generate-qr";

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
                let body = match response.bytes().await {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        eprintln!("Error reading QR code data: {}", e);
                        return;
                    }
                };

                let output_dir = "QR_CODES";
                std::fs::create_dir_all(output_dir).unwrap_or_else(|e| {
                    eprintln!("Error creating output directory: {}", e);
                    std::process::exit(1);
                });

                let file_name = format!("{}/qrcode.{}", output_dir, request_input.format);
                let mut file = match File::create(&file_name) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Error creating file: {}", e);
                        return;
                    }
                };

                if let Err(e) = file.write_all(&body) {
                    eprintln!("Error writing to file: {}", e);
                } else {
                    println!("QR Code saved successfully as {}", file_name);
                    println!("Thanks for using QR-Code: by @Nkwenti@Severian\n");
                }
            } else {
                eprintln!("Failed to generate QR Code. Status: {}", response.status());
            }
        }
        Err(e) => {
            eprintln!("Error occurred while sending request: {}", e);
        }
    }
}
