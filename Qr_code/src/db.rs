use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn connect() -> Pool<Postgres>{
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://halamadrid:mysecretpassword@localhost:5432/qr_code_db")
        .await
        .expect("Failed to connect to the database");
    pool
}
