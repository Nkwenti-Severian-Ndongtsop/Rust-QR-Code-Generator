use axum::{routing::get, Router};


async fn root() -> &'static str {
    "Hello Qr_code Running"
}
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:7878";
    let listener = tokio::net::TcpListener::bind(addr).await;
    let listener = match listener {
        Ok(listener) => {
            println!("The server is running on: http://{}", addr);
            listener
        }
        Err(e) => {
            eprint!("Couldn't bind addres: {}", e);
            return;
        }
    };
    
    let app = Router::new().route("/", get(root));
    let response = axum::serve(listener, app).await;

    match response {
        Ok(_) => {
            println!("Successful response from Server")
        }
        Err(e) => {
            eprintln!("Error response from server: {}", e)
        },
    }


}


