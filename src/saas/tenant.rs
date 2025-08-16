use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::api::auth::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantContext {
    pub user_id: String,
    pub tenant_id: String,
    pub email: String,
    pub role: UserRole,
    pub permissions: Vec<String>,
    pub token_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub tenant_id: String,
    pub email: String,
    pub role: String,
    pub permissions: Vec<String>,
    pub token_id: String,
    pub exp: usize,
    pub iat: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for TenantContext
where
    S: Send + Sync,
    PgPool: axum::extract::FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Check for Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Check for API key format
        if token.starts_with("sbr_key_") {
            return validate_api_key(token, state).await;
        }

        // Validate JWT token
        validate_jwt_token(token, state).await
    }
}

async fn validate_jwt_token<S>(token: &str, state: &S) -> Result<TenantContext, StatusCode>
where
    S: Send + Sync,
    PgPool: axum::extract::FromRef<S>,
{
    let db = PgPool::from_ref(state);

    // TODO: Get JWT secret from config
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret".to_string());
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    // Decode and validate JWT
    let token_data = decode::<Claims>(token, &decoding_key, &Validation::default())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = token_data.claims;

    // Check if token is blacklisted
    if is_token_blacklisted(&db, &claims.token_id).await? {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Parse role
    let role = match claims.role.as_str() {
        "Admin" => UserRole::Admin,
        "Developer" => UserRole::Developer,
        "Analyst" => UserRole::Analyst,
        "Viewer" => UserRole::Viewer,
        _ => return Err(StatusCode::FORBIDDEN),
    };

    Ok(TenantContext {
        user_id: claims.sub,
        tenant_id: claims.tenant_id,
        email: claims.email,
        role,
        permissions: claims.permissions,
        token_id: claims.token_id,
    })
}

async fn validate_api_key<S>(api_key: &str, state: &S) -> Result<TenantContext, StatusCode>
where
    S: Send + Sync,
    PgPool: axum::extract::FromRef<S>,
{
    let db = PgPool::from_ref(state);

    // Look up API key in database
    let key_info = get_api_key_info(&db, api_key)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check if key is expired
    if let Some(expires_at) = key_info.expires_at {
        if chrono::Utc::now() > expires_at {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    // Update last used timestamp
    let _ = update_api_key_last_used(&db, &key_info.key_id).await;

    Ok(TenantContext {
        user_id: key_info.user_id,
        tenant_id: key_info.tenant_id,
        email: key_info.email,
        role: key_info.role,
        permissions: key_info.permissions,
        token_id: key_info.key_id, // Use key_id as token_id for API keys
    })
}

// Helper types and functions

#[derive(Debug)]
pub struct ApiKeyInfo {
    pub key_id: String,
    pub user_id: String,
    pub tenant_id: String,
    pub email: String,
    pub role: UserRole,
    pub permissions: Vec<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

async fn is_token_blacklisted(db: &PgPool, token_id: &str) -> Result<bool, StatusCode> {
    // TODO: Check if token is in blacklist table
    Ok(false)
}

async fn get_api_key_info(db: &PgPool, api_key: &str) -> Result<ApiKeyInfo, Box<dyn std::error::Error>> {
    // TODO: Look up API key in database and return associated user/tenant info
    todo!()
}

async fn update_api_key_last_used(db: &PgPool, key_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Update last_used timestamp for API key
    Ok(())
}