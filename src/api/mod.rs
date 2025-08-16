pub mod auth;
pub mod recommendations;
pub mod admin;
pub mod middleware;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::api::middleware::auth::AuthMiddleware;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
    pub database: String,
    pub models_loaded: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub documentation_url: String,
    pub supported_languages: Vec<String>,
    pub supported_industries: Vec<String>,
}

pub struct AppState {
    pub db: PgPool,
}

pub fn create_api_router(db: PgPool) -> Router {
    let app_state = AppState { db };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Public routes (no auth required)
        .route("/health", get(health_check))
        .route("/info", get(api_info))
        
        // Authentication routes
        .nest("/auth", auth::create_auth_router())
        
        // Protected routes (require authentication)
        .nest("/api/v1", create_protected_routes())
        
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(AuthMiddleware::new())
        )
        .with_state(app_state)
}

fn create_protected_routes() -> Router<AppState> {
    Router::new()
        .nest("/recommendations", recommendations::create_recommendation_router())
        .nest("/admin", admin::create_admin_router())
}

/// Health check endpoint
pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, StatusCode> {
    // Check database connection
    let db_status = match sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
    {
        Ok(_) => "healthy".to_string(),
        Err(_) => "unhealthy".to_string(),
    };

    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        database: db_status,
        models_loaded: 0, // TODO: Count loaded models
    }))
}

/// API information endpoint
pub async fn api_info() -> Json<ApiInfo> {
    Json(ApiInfo {
        name: "SBR-RS Recommendation API".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: "Sequence-based recommendation system with LLM integration for Vietnamese language support".to_string(),
        documentation_url: "https://api.sbr-rs.com/docs".to_string(),
        supported_languages: vec![
            "en".to_string(),
            "vi".to_string(),
        ],
        supported_industries: vec![
            "ecommerce".to_string(),
            "media".to_string(),
            "education".to_string(),
            "finance".to_string(),
        ],
    })
}