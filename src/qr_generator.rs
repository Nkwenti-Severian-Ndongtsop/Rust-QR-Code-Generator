use qrcode::QrCode;
use image::Luma;
use reqwest::Error;
use std::io::Cursor;

pub fn generate_qr(data: &str, size: u32, format: &str) -> Result<Vec<u8>, Error> {
    let code = match QrCode::new(data.as_bytes()) {
        Ok(code) => {
            code
        },
        Err(e) => {
            eprintln!("Couldn't generate QR-Code: {}", e);
            return Ok(vec![])
        },
    };

    let image = code.render::<Luma<u8>>().min_dimensions(size, size).build();

    let form = match format {
        "webp" => image::ImageFormat::WebP,
        "jpeg" => image::ImageFormat::Jpeg,
        "bmp" => image::ImageFormat::Bmp,
        _ => image::ImageFormat::Png
    };

    let mut buffer = Cursor::new(Vec::new());
    let image = image
        .write_to(&mut buffer, form);
    match image {
        Ok(image) => {
            image
        },
        Err(e) => {
            eprint!("Error: {}", e)
        },
    }

    Ok(buffer.into_inner())
}
