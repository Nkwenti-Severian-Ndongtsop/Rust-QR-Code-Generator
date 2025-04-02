use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
mod qr_generator;

#[derive(Serialize, Deserialize)]
pub struct QRRequest {
    data: String,
    size: u32,
    format: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables

    let args: Vec<String> = env::args().collect();
    
    if args.len() != 4 {
        eprintln!("Usage: cargo run <data> <format> <size>");
        std::process::exit(1);
    }

    let data = args[1].clone();
    let format = args[2].clone();
    let size: u32 = match args[3].trim().parse() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Invalid size. Must be an integer.");
            std::process::exit(1);
        }
    };
    
    let request_input = QRRequest { data: data.clone(), format: format.clone(), size };

    // Generate QR Code
    match qr_generator::generate_qr(&data, size, &format) {
        Ok(body) => {
            let home_dir = dirs::home_dir().expect("Could not find home directory");
            let output_dir = home_dir.join("QR_CODES");
            std::fs::create_dir_all(&output_dir).expect("Error creating output directory");

            let file_path: PathBuf = output_dir.join(format!("qrcode.{}", format));
            let mut file = File::create(&file_path).expect("Error creating file");
            file.write_all(&body).expect("Error writing to file");

            println!("✅ QR Code saved successfully at {:?}", file_path);

            // Insert into database
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = match PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await {
                    Ok(res) => {
                        res
                    },
                    Err(_) => {
                        println!("\nThanks for using QR-Code: by @Nkwenti @Severian");
                        return;
                    }
                };
                

            match sqlx::query("INSERT INTO DATA (data_input, image_type, image_size) VALUES ($1, $2, $3)")
                .bind(&request_input.data)
                .bind(&request_input.format)
                .bind(request_input.size as i64)
                .execute(&pool)
                .await {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        return
                    }
                }
                

            println!("\nThanks for using QR-Code: by @Nkwenti @Severian");
        }
        Err(e) => {
            eprintln!("❌ Failed to generate QR Code. Error: {}", e);
            std::process::exit(1);
        }
    }
}
