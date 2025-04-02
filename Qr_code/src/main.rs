use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

mod auth;

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
    if args.len() != 4 {
        eprintln!("Usage: cargo run <data> <format> <size>");
        std::process::exit(1);
    }

    let size = match args[3].trim().parse::<u32>() {
        Ok(s) => Some(s),
        Err(_) => {
            eprintln!("Invalid size. Must be an integer.");
            std::process::exit(1);
        }
    };

    let request_input = QRRequest {
        data: args[1].clone(),
        format: args[2].clone(),
        size,
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

                // Get the home directory and create the QR_CODES folder
                let home_dir = dirs::home_dir().expect("Could not find home directory");
                let output_dir = home_dir.join("QR_CODES");

                if let Err(e) = std::fs::create_dir_all(&output_dir) {
                    eprintln!("Error creating output directory: {}", e);
                    std::process::exit(1);
                }

                // Save the file
                let file_path: PathBuf = output_dir.join(format!("qrcode.{}", request_input.format));
                let mut file = match File::create(&file_path) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Error creating file: {}", e);
                        return;
                    }
                };

                if let Err(e) = file.write_all(&body) {
                    eprintln!("Error writing to file: {}", e);
                    return;
                }

                // Insert into database
                if dotenv().is_err() {
                    eprintln!("Failed to load .env file.");
                    std::process::exit(1);
                }
                let database_url = match env::var("DATABASE_URL") {
                    Ok(url) => url,
                    Err(_) => {
                        eprintln!("DATABASE_URL environment variable not set.");
                        std::process::exit(1);
                    }
                };

                let pool = match sqlx::PgPool::connect(&database_url).await {
                    Ok(pool) => pool,
                    Err(e) => {
                        eprintln!("Failed to connect to the database: {}", e);
                        std::process::exit(1);
                    }
                };

                if let Err(e) = sqlx::query(
                    "INSERT INTO DATA (data_input, image_type, image_size) VALUES ($1, $2, $3)",
                )
                .bind(&request_input.data)
                .bind(&request_input.format)
                .bind(request_input.size.unwrap() as i64)
                .execute(&pool)
                .await
                {
                    eprintln!("Database error: {}", e);
                    return;
                }

                println!("\nQR Code saved successfully at {:?}", file_path);
                println!("\n\nThanks for using QR-Code: by @Nkwenti @Severian\n");
            } else {
                eprintln!("Failed to generate QR Code. Status: {}", response.status());
            }
        }
        Err(e) => {
            eprintln!("Error: Could not connect to server at {}. Ensure the server is running.", url);
            eprintln!("Details: {}", e);
        }
    }
}
