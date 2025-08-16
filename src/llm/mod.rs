pub mod vietnamese;
pub mod embeddings;
pub mod hybrid;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentFeatures {
    pub item_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub embedding: Option<Vec<f32>>,
    pub sentiment_score: Option<f32>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub model_name: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub vietnamese_enabled: bool,
    pub embedding_dimension: usize,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            model_name: "phobert-base".to_string(),
            max_tokens: 512,
            temperature: 0.1,
            vietnamese_enabled: true,
            embedding_dimension: 768,
        }
    }
}

#[derive(Debug)]
pub enum LLMError {
    ModelNotLoaded,
    TokenizationError(String),
    InferenceError(String),
    EmbeddingError(String),
}

impl std::fmt::Display for LLMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LLMError::ModelNotLoaded => write!(f, "LLM model not loaded"),
            LLMError::TokenizationError(e) => write!(f, "Tokenization error: {}", e),
            LLMError::InferenceError(e) => write!(f, "Inference error: {}", e),
            LLMError::EmbeddingError(e) => write!(f, "Embedding error: {}", e),
        }
    }
}

impl std::error::Error for LLMError {}

pub trait LLMService: Send + Sync {
    /// Generate embeddings for Vietnamese text
    async fn get_vietnamese_embeddings(
        &self,
        texts: &[String],
    ) -> Result<Vec<Vec<f32>>, LLMError>;

    /// Analyze sentiment of Vietnamese text
    async fn analyze_sentiment(
        &self,
        text: &str,
    ) -> Result<f32, LLMError>; // Returns score between -1.0 (negative) and 1.0 (positive)

    /// Extract features from product description
    async fn extract_content_features(
        &self,
        title: Option<&str>,
        description: Option<&str>,
    ) -> Result<ContentFeatures, LLMError>;

    /// Find similar items based on content
    async fn find_similar_items(
        &self,
        target_embedding: &[f32],
        candidate_embeddings: &[Vec<f32>],
        top_k: usize,
    ) -> Result<Vec<(usize, f32)>, LLMError>;

    /// Generate category predictions
    async fn predict_category(
        &self,
        title: &str,
        description: Option<&str>,
        existing_categories: &[String],
    ) -> Result<String, LLMError>;
}