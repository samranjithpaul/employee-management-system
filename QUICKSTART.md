# 🚀 Quick Start Guide - Employee Management System

## Option 1: Docker (Easiest - Recommended) 🐳

### Prerequisites
- Docker Desktop installed and running
- Docker Compose installed

### Steps:

1. **Navigate to the project directory:**
   ```bash
   cd ems_project
   ```

2. **Start all services (database, API, and frontend):**
   ```bash
   docker-compose up -d
   ```

3. **Wait for services to start (about 1-2 minutes):**
   ```bash
   # Check logs
   docker-compose logs -f
   ```
   
   Press `Ctrl+C` to exit logs. Services are ready when you see:
   - Database: "database system is ready to accept connections"
   - API: "Starting EMS API server on http://127.0.0.1:8000"
   - Streamlit: "You can now view your Streamlit app"

4. **Access the application:**
   - 🌐 **Streamlit UI**: http://localhost:8501
   - 🔌 **API**: http://localhost:8000
   - 🗄️ **PostgreSQL**: localhost:5432

5. **Login with default credentials:**
   - Email: `admin@ems.com`
   - Password: `admin123`

### Useful Docker Commands:

```bash
# View logs
docker-compose logs -f

# Stop all services
docker-compose down

# Stop and remove volumes (clean slate)
docker-compose down -v

# Restart services
docker-compose restart

# Rebuild after code changes
docker-compose up -d --build
```

---

## Option 2: Manual Setup (For Development) 🛠️

### Prerequisites
- PostgreSQL 15+ installed and running
- Rust (latest stable) - https://rustup.rs/
- Python 3.11+ and pip
- Cargo (comes with Rust)

### Step 1: Database Setup

```bash
# Start PostgreSQL service (macOS)
brew services start postgresql@15
# OR (Linux)
sudo systemctl start postgresql

# Create database
createdb ems_db

# Run schema
psql -d ems_db -f database/schema.sql

# Seed with initial data
psql -d ems_db -f database/seed.sql
```

### Step 2: Rust API Setup

```bash
cd rust_api

# Create .env file
cat > .env << EOF
DATABASE_URL=postgres://admin:password@localhost:5432/ems_db
JWT_SECRET=your-secret-key-change-in-production
EOF

# Build and run (first time will take a few minutes)
cargo run
```

The API will start on `http://localhost:8000`

**Keep this terminal open!**

### Step 3: Streamlit Frontend Setup

Open a **new terminal**:

```bash
cd streamlit_app

# Install Python dependencies
pip install -r requirements.txt

# Run Streamlit
streamlit run app.py
```

The app will start on `http://localhost:8501`

### Step 4: Access the Application

1. Open browser: http://localhost:8501
2. Login with:
   - Email: `admin@ems.com`
   - Password: `admin123`

---

## Troubleshooting 🔧

### Docker Issues:

**Port already in use:**
```bash
# Check what's using the port
lsof -i :8501  # Streamlit
lsof -i :8000  # API
lsof -i :5432  # PostgreSQL

# Stop conflicting services or change ports in docker-compose.yml
```

**Database connection errors:**
```bash
# Check if database container is running
docker-compose ps

# View database logs
docker-compose logs db

# Restart database
docker-compose restart db
```

**API not responding:**
```bash
# Check API logs
docker-compose logs api

# Rebuild API
docker-compose up -d --build api
```

### Manual Setup Issues:

**Rust compilation errors:**
```bash
# Update Rust
rustup update

# Clean and rebuild
cd rust_api
cargo clean
cargo build
```

**Python import errors:**
```bash
# Make sure you're in the right directory
cd streamlit_app

# Reinstall dependencies
pip install -r requirements.txt --force-reinstall
```

**Database connection:**
- Verify PostgreSQL is running: `psql -l`
- Check DATABASE_URL in `.env` matches your PostgreSQL setup
- Default: `postgres://admin:password@localhost:5432/ems_db`

---

## Testing the Setup ✅

### Test API:
```bash
# Test login endpoint
curl -X POST http://localhost:8000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@ems.com","password":"admin123"}'
```

You should get a JSON response with a `token` field.

### Test Database:
```bash
# Connect to database
psql -d ems_db

# Check tables
\dt

# Check admin exists
SELECT email FROM admins;

# Exit
\q
```

---

## Next Steps 📝

1. ✅ Login to the application
2. ✅ View the dashboard with sample employees
3. ✅ Add a new employee
4. ✅ View employee details
5. ✅ Upload a payslip
6. ✅ Add employment history
7. ✅ Test search and filters

---

## Need Help? 🆘

- Check the main README.md for detailed documentation
- Review logs: `docker-compose logs` or check terminal output
- Verify all services are running: `docker-compose ps`

Happy coding! 🎉

