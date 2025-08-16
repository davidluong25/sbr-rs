# API Documentation - SBR-RS SaaS Platform

## Tổng Quan

SBR-RS API cung cấp các endpoint RESTful để tích hợp hệ thống gợi ý sản phẩm vào ứng dụng của bạn. API hỗ trợ authentication qua JWT và API keys, với khả năng multi-tenancy đầy đủ.

## Base URL
```
Production: https://api.sbr-rs.com
Staging: https://staging-api.sbr-rs.com
Local: http://localhost:3000
```

## Authentication

### JWT Authentication (Recommended)
```bash
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" \
     https://api.sbr-rs.com/api/v1/recommendations/recommend
```

### API Key Authentication
```bash
curl -H "Authorization: Bearer sbr_key_your_api_key_here" \
     https://api.sbr-rs.com/api/v1/recommendations/recommend
```

## Endpoints

### 1. Authentication

#### POST /auth/register
Đăng ký tenant mới (công ty/tổ chức)

**Request Body:**
```json
{
  "email": "admin@company.com",
  "password": "secure_password123",
  "company_name": "My E-commerce Company",
  "industry": "ecommerce",
  "full_name": "John Doe"
}
```

**Response:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
  "message": "Registration successful",
  "verification_required": false
}
```

#### POST /auth/login
Đăng nhập và nhận JWT token

**Request Body:**
```json
{
  "email": "admin@company.com",
  "password": "secure_password123"
}
```

**Response:**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 3600,
  "user_info": {
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "admin@company.com",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "role": "Admin",
    "permissions": ["read:recommendations", "write:models", "admin:all"]
  }
}
```

### 2. Recommendations

#### POST /api/v1/recommendations/recommend
Lấy gợi ý sản phẩm cho người dùng

**Headers:**
```
Authorization: Bearer YOUR_JWT_TOKEN
Content-Type: application/json
```

**Request Body:**
```json
{
  "user_id": "user123",
  "num_recommendations": 10,
  "exclude_items": ["item456", "item789"],
  "context": {
    "category": "electronics",
    "price_range": "100-500",
    "location": "hanoi"
  }
}
```

**Response:**
```json
{
  "recommendations": [
    {
      "item_id": "item123",
      "score": 0.95,
      "confidence": 0.87,
      "reason": "Dựa trên người dùng tương tự"
    },
    {
      "item_id": "item456",
      "score": 0.89,
      "confidence": 0.82,
      "reason": "Trending trong danh mục của bạn"
    }
  ],
  "model_version": "1.0.0",
  "request_id": "req_550e8400-e29b-41d4-a716-446655440000",
  "processing_time_ms": 45
}
```

#### POST /api/v1/recommendations/batch
Lấy gợi ý cho nhiều người dùng cùng lúc

**Request Body:**
```json
{
  "user_ids": ["user1", "user2", "user3"],
  "num_recommendations": 5,
  "common_excludes": ["item999"]
}
```

**Response:**
```json
{
  "results": {
    "user1": [
      {
        "item_id": "item123",
        "score": 0.95,
        "confidence": 0.87,
        "reason": "Collaborative filtering"
      }
    ],
    "user2": [...],
    "user3": [...]
  },
  "request_id": "batch_req_123",
  "processing_time_ms": 120
}
```

#### POST /api/v1/recommendations/feedback
Gửi feedback về chất lượng gợi ý

**Request Body:**
```json
{
  "user_id": "user123",
  "item_id": "item456",
  "feedback_type": "Purchase",
  "rating": 4.5,
  "implicit": false
}
```

### 3. Model Management

#### POST /api/v1/recommendations/train
Bắt đầu training model mới

**Request Body:**
```json
{
  "dataset_id": "dataset123",
  "model_type": "Hybrid",
  "hyperparameters": {
    "learning_rate": 0.01,
    "embedding_dim": 128,
    "num_epochs": 50,
    "lstm_hidden_size": 64
  }
}
```

**Response:**
```json
{
  "job_id": "job_550e8400-e29b-41d4-a716-446655440000",
  "status": "queued",
  "estimated_completion": "2024-01-15T10:30:00Z"
}
```

#### GET /api/v1/recommendations/models/{model_id}/status
Kiểm tra trạng thái training model

**Response:**
```json
{
  "model_id": "model123",
  "status": "training",
  "accuracy": 0.85,
  "last_trained": "2024-01-15T08:00:00Z",
  "training_progress": 0.75
}
```

### 4. Admin

#### GET /api/v1/admin/stats
Thống kê tổng quan của tenant

**Response:**
```json
{
  "total_users": 150,
  "total_models": 5,
  "total_interactions": 50000,
  "active_models": 3,
  "api_calls_today": 2500
}
```

#### GET /api/v1/admin/models
Danh sách tất cả models của tenant

**Response:**
```json
[
  {
    "id": "model123",
    "name": "E-commerce LSTM v1.0",
    "model_type": "LSTM",
    "version": "1.0.0",
    "status": "ready",
    "accuracy": 0.87,
    "created_at": "2024-01-10T08:00:00Z",
    "is_active": true
  }
]
```

### 5. Utilities

#### GET /health
Health check endpoint

**Response:**
```json
{
  "status": "ok",
  "version": "1.0.0",
  "timestamp": "2024-01-15T10:00:00Z",
  "database": "healthy",
  "models_loaded": 3
}
```

#### GET /info
Thông tin API

**Response:**
```json
{
  "name": "SBR-RS Recommendation API",
  "version": "1.0.0",
  "description": "Sequence-based recommendation system with LLM integration for Vietnamese language support",
  "documentation_url": "https://api.sbr-rs.com/docs",
  "supported_languages": ["en", "vi"],
  "supported_industries": ["ecommerce", "media", "education", "finance"]
}
```

## Response Codes

| Code | Description |
|------|-------------|
| 200  | Success |
| 201  | Created |
| 400  | Bad Request - Invalid input |
| 401  | Unauthorized - Invalid or missing token |
| 403  | Forbidden - Insufficient permissions |
| 404  | Not Found |
| 409  | Conflict - Resource already exists |
| 429  | Too Many Requests - Rate limited |
| 500  | Internal Server Error |

## Error Response Format

```json
{
  "error": {
    "code": "INVALID_INPUT",
    "message": "User ID is required",
    "details": {
      "field": "user_id",
      "reason": "missing_field"
    }
  },
  "request_id": "req_123456"
}
```

## Rate Limits

| Tier | Requests/Minute | Daily Limit |
|------|----------------|-------------|
| Startup | 100 | 10,000 |
| Growth | 500 | 50,000 |
| Enterprise | 2,000 | 500,000+ |

## Vietnamese Language Support

API tự động phát hiện và xử lý nội dung tiếng Việt:

- **Phân tích sentiment**: Tự động phân tích cảm xúc trong feedback và reviews
- **Phân loại sản phẩm**: Tự động categorize sản phẩm dựa trên tên và mô tả
- **Tìm kiếm semantic**: Hỗ trợ tìm kiếm và matching dựa trên nghĩa
- **Từ khóa extraction**: Trích xuất từ khóa quan trọng từ text tiếng Việt

### Ví dụ với tiếng Việt

**Request:**
```json
{
  "user_id": "user_vietnam",
  "num_recommendations": 5,
  "context": {
    "product_category": "thời trang",
    "description": "Tìm áo sơ mi nam công sở đẹp, chất lượng tốt"
  }
}
```

**Response sẽ ưu tiên:**
- Sản phẩm với category "thời trang"
- Items có keywords: "áo sơ mi", "nam", "công sở"
- Sản phẩm có rating và reviews tích cực

## SDKs và Integration

### JavaScript/Node.js
```javascript
const SbrClient = require('@sbr-rs/client');

const client = new SbrClient({
  apiKey: 'your_api_key',
  baseURL: 'https://api.sbr-rs.com'
});

const recommendations = await client.getRecommendations({
  userId: 'user123',
  numRecommendations: 10
});
```

### Python
```python
from sbr_rs import SbrClient

client = SbrClient(
    api_key='your_api_key',
    base_url='https://api.sbr-rs.com'
)

recommendations = client.get_recommendations(
    user_id='user123',
    num_recommendations=10
)
```

### PHP
```php
use SbrRs\Client;

$client = new Client([
    'api_key' => 'your_api_key',
    'base_url' => 'https://api.sbr-rs.com'
]);

$recommendations = $client->getRecommendations([
    'user_id' => 'user123',
    'num_recommendations' => 10
]);
```

## Support & Documentation

- **Documentation**: https://docs.sbr-rs.com
- **Support**: support@sbr-rs.com
- **GitHub**: https://github.com/davidluong25/sbr-rs
- **Status Page**: https://status.sbr-rs.com