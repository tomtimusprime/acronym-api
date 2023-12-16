use std::net::SocketAddr;

use axum::{Router, Extension};
mod db;
mod rest;
use db::{init_db};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env if available
    dotenv::dotenv().ok();

    // Initialize the database and obtain a connection pool
    let connection_pool = init_db().await?;

    // Initialize the Axum routing service
    let app = Router::new()
    .nest_service("/acronyms", rest::acronym_service())
    .layer(Extension(connection_pool));

    // Define the address to listen on (everything)
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Server running on {:?}", addr);
    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}