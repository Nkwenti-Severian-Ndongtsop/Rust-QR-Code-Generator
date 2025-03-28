
mod user_input;
mod db;
mod auth;
mod qr_generator;
mod routes;

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
    
    let response = axum::serve(listener, routes::create_routes()).await;

    match response {
        Ok(_) => {
            println!("Successful response from Server")
        }
        Err(e) => {
            eprintln!("Error response from server: {}", e)
        },
    }


}


