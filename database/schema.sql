-- Employee Management System Database Schema

-- Table: admins
CREATE TABLE IF NOT EXISTS admins (
    admin_id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Table: employees
CREATE TABLE IF NOT EXISTS employees (
    emp_id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    department TEXT,
    designation TEXT,
    joining_date DATE,
    status TEXT DEFAULT 'Active',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Table: employment_history
CREATE TABLE IF NOT EXISTS employment_history (
    history_id SERIAL PRIMARY KEY,
    emp_id INT REFERENCES employees(emp_id) ON DELETE CASCADE,
    company_name TEXT NOT NULL,
    start_date DATE,
    end_date DATE,
    position TEXT,
    payslip_pdf_path TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Table: audit_logs
CREATE TABLE IF NOT EXISTS audit_logs (
    log_id SERIAL PRIMARY KEY,
    admin_id INT REFERENCES admins(admin_id) ON DELETE SET NULL,
    emp_id INT REFERENCES employees(emp_id) ON DELETE SET NULL,
    action TEXT NOT NULL,
    details TEXT,
    timestamp TIMESTAMP DEFAULT NOW()
);

-- Indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_employees_email ON employees(email);
CREATE INDEX IF NOT EXISTS idx_employees_department ON employees(department);
CREATE INDEX IF NOT EXISTS idx_employees_status ON employees(status);
CREATE INDEX IF NOT EXISTS idx_employment_history_emp_id ON employment_history(emp_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_admin_id ON audit_logs(admin_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_emp_id ON audit_logs(emp_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_timestamp ON audit_logs(timestamp);

