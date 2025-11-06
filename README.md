# 🧠 Employee Management System (EMS)

A Full-Stack Employee Management Platform built using **Streamlit**, **Rust**, and **PostgreSQL**.

---

### 👨‍💻 Developer Information

**Name:** Sam Ranjith Paul  

**GitHub:** [github.com/samranjithpaul](https://github.com/samranjithpaul)  

**LinkedIn:** [linkedin.com/in/Samranjithpaul](https://www.linkedin.com/in/Samranjithpaul)  

**Role:** Full-Stack Developer (Python · Rust · SQL)

---

© 2025 Sam Ranjith Paul — All rights reserved.  

Unauthorized removal of developer credits is strictly prohibited.

---

## 🎯 Features

- 🔐 **JWT Authentication** - Secure admin login and session management
- 👥 **Employee CRUD** - Create, read, update, and delete employee records
- 📜 **Employment History** - Track previous employment records
- 💰 **Payslip Management** - Upload and manage employee payslip PDFs
- 🔍 **Search & Filter** - Advanced filtering by department, designation, status, and search
- 📊 **Dashboard** - Comprehensive employee overview with statistics
- 📝 **Audit Logging** - Track all admin actions for compliance

## 🏗️ Architecture

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│  Streamlit  │─────▶│  Rust API   │─────▶│ PostgreSQL  │
│  Frontend   │      │  (Actix)    │      │  Database   │
└─────────────┘      └─────────────┘      └─────────────┘
```

## 📁 Project Structure

```
ems_project/
├── streamlit_app/          # Streamlit frontend
│   ├── app.py             # Main app entry point
│   ├── pages/             # Streamlit pages
│   │   ├── login.py
│   │   ├── dashboard.py
│   │   ├── add_employee.py
│   │   ├── employee_detail.py
│   │   └── upload_payslip.py
│   ├── utils/             # Utility modules
│   │   ├── api_client.py
│   │   └── auth.py
│   └── uploads/           # Uploaded files
│
├── rust_api/              # Rust backend API
│   ├── src/
│   │   ├── main.rs
│   │   ├── models.rs
│   │   ├── db.rs
│   │   └── routes/
│   │       ├── auth.rs
│   │       ├── employee.rs
│   │       ├── history.rs
│   │       └── payslip.rs
│   └── Cargo.toml
│
├── database/              # Database scripts
│   ├── schema.sql
│   └── seed.sql
│
└── docker-compose.yml     # Docker orchestration
```

## 🚀 Quick Start

### Prerequisites

- Docker and Docker Compose
- OR: Rust, Python 3.11+, PostgreSQL 15

### Option 1: Docker (Recommended)

1. **Clone and navigate to the project:**
   ```bash
   cd ems_project
   ```

2. **Start all services:**
   ```bash
   docker-compose up -d
   ```

3. **Access the application:**
   - Streamlit UI: http://localhost:8501
   - Rust API: http://localhost:8000
   - PostgreSQL: localhost:5432

4. **Default Admin Credentials:**
   - Email: `admin@ems.com`
   - Password: `admin123`

### Option 2: Manual Setup

#### 1. Database Setup

```bash
# Start PostgreSQL
# Create database
createdb ems_db

# Run schema
psql -d ems_db -f database/schema.sql

# Seed data
psql -d ems_db -f database/seed.sql
```

#### 2. Rust API Setup

```bash
cd rust_api

# Create .env file
echo "DATABASE_URL=postgres://admin:password@localhost:5432/ems_db" > .env
echo "JWT_SECRET=your-secret-key-change-in-production" >> .env

# Build and run
cargo build --release
cargo run
```

The API will run on `http://localhost:8000`

#### 3. Streamlit App Setup

```bash
cd streamlit_app

# Install dependencies
pip install -r requirements.txt

# Run Streamlit
streamlit run app.py
```

The app will run on `http://localhost:8501`

## 📡 API Endpoints

### Authentication
- `POST /auth/register` - Register new admin
- `POST /auth/login` - Login and get JWT token

### Employees
- `GET /employees` - Get all employees
- `GET /employees/{id}` - Get employee by ID
- `POST /employees` - Create new employee (requires auth)
- `PUT /employees/{id}` - Update employee (requires auth)
- `DELETE /employees/{id}` - Delete employee (requires auth)

### Employment History
- `GET /employees/{id}/history` - Get employment history
- `POST /employees/{id}/history` - Add employment history (requires auth)

### Payslips
- `POST /employees/{id}/payslip` - Upload payslip PDF (requires auth)
- `GET /employees/{id}/payslips` - List payslips
- `GET /payslip/{filename}` - Download payslip

### Audit Logs
- `GET /audit_logs` - Get audit logs (requires auth)

## 🔧 Configuration

### Environment Variables

**Rust API (.env):**
```env
DATABASE_URL=postgres://admin:password@localhost:5432/ems_db
JWT_SECRET=your-secret-key-change-in-production
```

**Streamlit:**
Update `BASE_URL` in `utils/api_client.py` if API is not on `localhost:8000`

## 🗄️ Database Schema

### Tables

- **admins** - Admin user accounts
- **employees** - Employee records
- **employment_history** - Previous employment records
- **audit_logs** - Admin action logs

See `database/schema.sql` for full schema details.

## 🧪 Testing

### Test API Endpoints

```bash
# Login
curl -X POST http://localhost:8000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@ems.com","password":"admin123"}'

# Get employees (replace TOKEN with JWT from login)
curl -X GET http://localhost:8000/employees \
  -H "Authorization: Bearer TOKEN"
```

## 🐳 Docker Commands

```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Rebuild and restart
docker-compose up -d --build

# Access database
docker exec -it ems_db psql -U admin -d ems_db
```

## 📝 Development Notes

### Adding New Features

1. **Backend (Rust):**
   - Add route handlers in `rust_api/src/routes/`
   - Update `main.rs` to register routes
   - Add models in `models.rs` if needed

2. **Frontend (Streamlit):**
   - Create new page in `streamlit_app/pages/`
   - Add API client functions in `utils/api_client.py`
   - Update navigation in `dashboard.py`

### Database Migrations

For production, consider using proper migration tools like:
- Diesel migrations
- SQLx migrations
- Or manual SQL scripts

## 🔒 Security Considerations

- Change default JWT secret in production
- Use strong passwords for admin accounts
- Enable HTTPS in production
- Implement rate limiting
- Add input validation and sanitization
- Use environment variables for secrets

## 🐛 Troubleshooting

### API Connection Issues
- Ensure Rust API is running on port 8000
- Check `BASE_URL` in `api_client.py`
- Verify CORS settings in `main.rs`

### Database Connection Issues
- Verify PostgreSQL is running
- Check `DATABASE_URL` in `.env`
- Ensure database exists and schema is applied

### File Upload Issues
- Check `uploads/payslips/` directory permissions
- Verify file size limits
- Check API logs for errors

## 📄 License

This project is provided as-is for educational and development purposes.

## 👥 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📞 Support

For issues and questions, please open an issue in the repository.

---

## 🧑‍💻 Developer Information

**Name:** Sam Ranjith Paul  

**GitHub:** [github.com/samranjithpaul](https://github.com/samranjithpaul)  

**LinkedIn:** [linkedin.com/in/Samranjithpaul](https://www.linkedin.com/in/Samranjithpaul)  

> All rights reserved © 2025 Sam Ranjith Paul  

> Unauthorized removal of credit or developer attribution is prohibited.

---

**Built with ❤️ by Sam Ranjith Paul using Streamlit, Rust, and PostgreSQL**

