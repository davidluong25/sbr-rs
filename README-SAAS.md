# SBR-RS SaaS Platform

## Overview
SBR-RS là một hệ thống gợi ý sản phẩm tiên tiến được nâng cấp thành SaaS platform với tích hợp LLM cho tiếng Việt.

## Tính Năng Mới

### ✨ LLM Integration
- Hỗ trợ tiếng Việt với PhoBERT embeddings
- Phân tích sentiment cho feedback
- Auto-categorization cho sản phẩm mới
- Content-based recommendations

### 🏢 Multi-Tenancy
- Tenant isolation với Row Level Security
- API key management
- Usage analytics per tenant
- Flexible billing plans

### 🔐 Authentication & Authorization
- JWT-based authentication
- Role-based access control (RBAC)
- API key authentication
- Secure password hashing

### 📊 Admin Dashboard
- Real-time analytics
- Model performance monitoring
- User management
- Training job tracking

### 🌐 REST API
- Recommendation endpoints
- Batch processing support
- Feedback collection
- Model training management

## Quick Start

### Prerequisites
- Rust 1.70+
- PostgreSQL 15+
- Redis 7+ (optional)

### Installation

```bash
# Clone repository
git clone https://github.com/davidluong25/sbr-rs.git
cd sbr-rs

# Setup database
createdb sbr_rs
export DATABASE_URL="postgresql://localhost/sbr_rs"

# Install dependencies and run migrations
cargo install sqlx-cli
sqlx migrate run

# Start the server
cargo run
```

### API Usage

```bash
# Health check
curl http://localhost:3000/health

# Register new tenant
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@company.com",
    "password": "secure_password",
    "company_name": "My Company",
    "industry": "ecommerce",
    "full_name": "Admin User"
  }'

# Get recommendations
curl -X POST http://localhost:3000/api/v1/recommendations/recommend \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "user123",
    "num_recommendations": 10
  }'
```

## Architecture

### Core Components
1. **API Layer** - REST endpoints with authentication
2. **Business Logic** - Recommendation algorithms
3. **Data Layer** - PostgreSQL with tenant isolation
4. **LLM Integration** - Vietnamese text processing
5. **Admin Interface** - Management dashboard

### Supported Industries
- **E-commerce**: Product recommendations, cross-selling
- **Media**: Content recommendation, playlist generation
- **Education**: Course recommendations, learning paths
- **Finance**: Investment advice, product suggestions

## Vietnamese Language Features

### Text Processing
- Vietnamese tokenization and normalization
- Stop words removal
- Sentiment analysis
- Keyword extraction

### Product Categorization
Automatic Vietnamese product categorization:
- Thời trang (Fashion)
- Điện tử (Electronics)
- Gia đình & Nhà cửa (Home & Living)
- Sách & Giáo dục (Books & Education)
- Sức khỏe (Health & Beauty)
- Thể thao (Sports)
- Ô tô & Xe máy (Automotive)

## Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature-name`
3. Commit changes: `git commit -am 'Add feature'`
4. Push branch: `git push origin feature-name`
5. Create Pull Request

## License

MIT License - see LICENSE file for details.

## Documentation

- [Kiến Trúc Hệ Thống](docs/vi/KIẾN-TRÚC-HỆ-THỐNG.md)
- [Kế Hoạch Phát Triển](plan/KẾ-HOẠCH-PHÁT-TRIỂN.md)
- [Cấu Trúc Dự Án](docs/vi/CẤU-TRÚC-DỰ-ÁN.md)
- [API Reference](https://api.sbr-rs.com/docs)

## Support

For support and questions:
- Email: support@sbr-rs.com
- Issues: [GitHub Issues](https://github.com/davidluong25/sbr-rs/issues)
- Documentation: [Docs Site](https://docs.sbr-rs.com)