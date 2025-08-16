use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio;
use tracing::{info, Level};
use tracing_subscriber;

// Use the library modules
use sbr::api;
use sbr::saas;
use sbr::llm;
use sbr::database;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Load configuration
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/sbr_rs".to_string());
    
    let bind_address = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    // Setup database connection pool
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    // Note: Migrations are commented out until we can properly set them up
    // info!("Running database migrations...");
    // sqlx::migrate!("./src/database/migrations").run(&db).await?;

    // Create API router
    let app = api::create_api_router(db);

    // Parse bind address
    let addr: SocketAddr = bind_address.parse()?;

    info!("Starting SBR-RS API server on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}