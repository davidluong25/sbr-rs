# SBR-RS API Server Implementation

## Cấu Trúc Dự Án Mới

```
src/
├── lib.rs              # Original library exports  
├── main.rs             # New API server entry point
├── api/                # REST API layer
│   ├── mod.rs
│   ├── auth.rs         # Authentication endpoints
│   ├── recommendations.rs  # Core recommendation API
│   ├── admin.rs        # Admin management API
│   └── middleware/     # Custom middleware
├── saas/               # SaaS-specific features
│   ├── mod.rs
│   ├── tenant.rs       # Multi-tenancy management
│   ├── billing.rs      # Billing integration
│   └── analytics.rs    # Usage analytics
├── llm/                # LLM integration
│   ├── mod.rs
│   ├── vietnamese.rs   # Vietnamese text processing
│   ├── embeddings.rs   # Text embeddings
│   └── hybrid.rs       # Hybrid model implementation
├── database/           # Database layer
│   ├── mod.rs
│   ├── models.rs       # Database models
│   ├── migrations/     # SQL migrations
│   └── repositories/   # Data access layer
├── models/             # Original ML models (enhanced)
│   ├── mod.rs
│   ├── lstm.rs         # Enhanced LSTM
│   ├── ewma.rs         # Enhanced EWMA
│   ├── hybrid.rs       # New hybrid model
│   └── sequence_model.rs
├── data.rs             # Original data structures
├── datasets.rs         # Original dataset utilities
└── evaluation.rs       # Original evaluation metrics
```