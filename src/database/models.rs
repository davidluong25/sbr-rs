use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub industry: String,
    pub settings: serde_json::Value,
    pub plan_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub role: String,
    pub is_active: bool,
    pub email_verified: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Model {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub model_type: String,
    pub version: String,
    pub status: String,
    pub hyperparameters: serde_json::Value,
    pub metrics: serde_json::Value,
    pub file_path: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Interaction {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: String,
    pub item_id: String,
    pub interaction_type: String,
    pub rating: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub context: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub external_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub price: Option<rust_decimal::Decimal>,
    pub metadata: serde_json::Value,
    // embedding: Vec<f32>, // This would require pgvector support
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}