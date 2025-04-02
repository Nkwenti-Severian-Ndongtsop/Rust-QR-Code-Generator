use db::connect;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

mod auth;
mod db;

#[derive(Serialize, Deserialize)]
pub struct QRRequest {
    data: String,
    size: Option<u32>,
    format: String,
}

#[tokio::main]
async fn main() {
    let url = "http://127.0.0.1:7878/generate-qr";

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: cargo run <data> <format> <size>");
        return;
    }

    let request_input = QRRequest {
        data: args[1].clone(),
        format: args[2].clone(),
        size: Some(args[3].trim().parse().expect("Invalid size")),
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

                let output_dir = "~/QR_CODES";
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
                    return eprintln!("Error writing to file: {}", e);
                } else {
                    let pool = connect().await;

                    match sqlx::query(
                        "INSERT INTO DATA (data_input, image_type, image_size) VALUES ($1, $2, $3)",
                    )
                    .bind(request_input.data)
                    .bind(request_input.format)
                    .bind(request_input.size.unwrap() as i64)
                    .execute(&pool)
                    .await {
                        Ok(_) => {},
                        Err(e) => return eprintln!("Error: {}", e),
                    }

                    println!("QR Code saved successfully as {}", file_name);
                    println!("Thanks for using QR-Code: by @Nkwenti@Severian\n");
                }
            } else {
                return eprintln!("Failed to generate QR Code. Status: {}", response.status());
            }
        }
        Err(e) => {
            return eprintln!("Error occurred while sending request: {}", e);
        }
    }
}
