// ================================================================
//  Employee Management System (EMS)
//  Developed by: Sam Ranjith Paul
//  GitHub: https://github.com/samranjithpaul
//  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
//  Unauthorized removal of this header is prohibited.
// ================================================================

use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Admin {
    pub admin_id: i32,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminRegister {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub emp_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department: Option<String>,
    pub designation: Option<String>,
    pub joining_date: Option<NaiveDate>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeCreate {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department: Option<String>,
    pub designation: Option<String>,
    pub joining_date: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeUpdate {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department: Option<String>,
    pub designation: Option<String>,
    pub joining_date: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmploymentHistory {
    pub history_id: i32,
    pub emp_id: i32,
    pub company_name: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub role: Option<String>,
    pub payslip_pdf_path: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentHistoryCreate {
    pub company_name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLog {
    pub log_id: i32,
    pub admin_id: Option<i32>,
    pub emp_id: Option<i32>,
    pub action: String,
    pub details: Option<String>,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: String, data: T) -> Self {
        Self {
            status: "success".to_string(),
            message,
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            status: "error".to_string(),
            message,
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub admin_id: i32,
    pub email: String,
    pub exp: usize,
}

