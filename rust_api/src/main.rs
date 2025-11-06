// ================================================================
//  Employee Management System (EMS)
//  Developed by: Sam Ranjith Paul
//  GitHub: https://github.com/samranjithpaul
//  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
//  Unauthorized removal of this header is prohibited.
// ================================================================

mod db;
mod models;
mod routes;

use actix_web::{web, App, HttpServer, middleware, HttpResponse};
use actix_cors::Cors;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde_json::json;

use db::establish_connection;
use models::Claims;
use routes::auth;
use routes::employee;
use routes::history;
use routes::payslip;

const JWT_SECRET: &str = "your-secret-key-change-in-production";

// Helper function to extract admin_id from JWT token
pub fn extract_admin_id(req: &actix_web::HttpRequest) -> Result<i32, actix_web::Error> {
    let auth_header = req.headers().get("Authorization");
    
    if let Some(header_value) = auth_header {
        if let Ok(auth_str) = header_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                
                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(JWT_SECRET.as_ref()),
                    &Validation::new(Algorithm::HS256),
                );
                
                if let Ok(data) = token_data {
                    return Ok(data.claims.admin_id);
                }
            }
        }
    }
    
    Err(actix_web::error::ErrorUnauthorized("Invalid or missing token"))
}

// Wrapper functions for protected routes
async fn create_employee_wrapper(
    req: actix_web::HttpRequest,
    pool: web::Data<db::DbPool>,
    data: web::Json<models::EmployeeCreate>,
) -> actix_web::HttpResponse {
    let admin_id = match extract_admin_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into(),
    };
    employee::create_employee(pool, data, admin_id).await
}

async fn update_employee_wrapper(
    req: actix_web::HttpRequest,
    pool: web::Data<db::DbPool>,
    path: web::Path<i32>,
    data: web::Json<models::EmployeeUpdate>,
) -> actix_web::HttpResponse {
    let admin_id = match extract_admin_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into(),
    };
    employee::update_employee(pool, path, data, admin_id).await
}

async fn delete_employee_wrapper(
    req: actix_web::HttpRequest,
    pool: web::Data<db::DbPool>,
    path: web::Path<i32>,
) -> actix_web::HttpResponse {
    let admin_id = match extract_admin_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into(),
    };
    employee::delete_employee(pool, path, admin_id).await
}

async fn add_history_wrapper(
    req: actix_web::HttpRequest,
    pool: web::Data<db::DbPool>,
    path: web::Path<i32>,
    data: web::Json<models::EmploymentHistoryCreate>,
) -> actix_web::HttpResponse {
    let admin_id = match extract_admin_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into(),
    };
    history::add_employment_history(pool, path, data, admin_id).await
}

async fn upload_payslip_wrapper(
    req: actix_web::HttpRequest,
    pool: web::Data<db::DbPool>,
    path: web::Path<i32>,
    payload: actix_multipart::Multipart,
) -> actix_web::Result<actix_web::HttpResponse> {
    let admin_id = extract_admin_id(&req)?;
    payslip::upload_payslip(pool, path, payload, admin_id).await
}

async fn get_audit_logs_wrapper(
    _req: actix_web::HttpRequest,
    pool: web::Data<db::DbPool>,
) -> actix_web::HttpResponse {
    use diesel::prelude::*;
    use diesel::sql_types::{Integer, Nullable, Text, Timestamp};
    
    // Admin ID already validated by extract_admin_id in the route wrapper
    
    #[derive(diesel::QueryableByName)]
    struct AuditLogRow {
        #[diesel(sql_type = Integer)]
        log_id: i32,
        #[diesel(sql_type = Nullable<Integer>)]
        admin_id: Option<i32>,
        #[diesel(sql_type = Nullable<Integer>)]
        emp_id: Option<i32>,
        #[diesel(sql_type = Text)]
        action: String,
        #[diesel(sql_type = Nullable<Text>)]
        details: Option<String>,
        #[diesel(sql_type = Timestamp)]
        timestamp: chrono::NaiveDateTime,
    }

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return actix_web::HttpResponse::InternalServerError().json(models::ApiResponse::<()>::error(
                "Database connection failed".to_string(),
            ));
        }
    };

    let result: Result<Vec<AuditLogRow>, _> = diesel::sql_query(
        "SELECT log_id, admin_id, emp_id, action, details, timestamp FROM audit_logs ORDER BY timestamp DESC LIMIT 100"
    )
    .load(&mut conn);

    match result {
        Ok(rows) => {
            let logs: Vec<models::AuditLog> = rows.into_iter().map(|row| models::AuditLog {
                log_id: row.log_id,
                admin_id: row.admin_id,
                emp_id: row.emp_id,
                action: row.action,
                details: row.details,
                timestamp: row.timestamp,
            }).collect();
            
            actix_web::HttpResponse::Ok().json(models::ApiResponse::success(
                "Audit logs retrieved successfully".to_string(),
                logs,
            ))
        }
        Err(e) => {
            eprintln!("Error fetching audit logs: {}", e);
            actix_web::HttpResponse::InternalServerError().json(models::ApiResponse::<()>::error(
                "Failed to fetch audit logs".to_string(),
            ))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::io::Write;
    let _ = std::io::stderr().write_all(b"Initializing EMS API...\n");
    let _ = std::io::stdout().write_all(b"Initializing EMS API...\n");
    
    // Initialize database pool
    let _ = std::io::stderr().write_all(b"Connecting to database...\n");
    let pool = establish_connection();
    let _ = std::io::stderr().write_all(b"Database connection pool created successfully\n");
    
    // Create uploads directory
    std::fs::create_dir_all("./uploads/payslips").unwrap_or(());

    eprintln!("Starting EMS API server on http://0.0.0.0:8000");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(auth::register))
                    .route("/login", web::post().to(auth::login)),
            )
            .service(
                web::scope("/employees")
                    .route("", web::get().to(employee::get_employees))
                    .route("", web::post().to(create_employee_wrapper))
                    .route("/{id}", web::get().to(employee::get_employee))
                    .route("/{id}", web::put().to(update_employee_wrapper))
                    .route("/{id}", web::delete().to(delete_employee_wrapper))
                    .route("/{id}/history", web::get().to(history::get_employment_history))
                    .route("/{id}/history", web::post().to(add_history_wrapper))
                    .route("/{id}/payslip", web::post().to(upload_payslip_wrapper))
                    .route("/{id}/payslips", web::get().to(payslip::list_payslips)),
            )
            .route("/payslip/{filename}", web::get().to(payslip::get_payslip))
            .route("/audit_logs", web::get().to(get_audit_logs_wrapper))
            .route("/meta", web::get().to(meta))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

// Metadata endpoint - returns developer information
async fn meta() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "project": "Employee Management System",
        "developer": "Sam Ranjith Paul",
        "github": "https://github.com/samranjithpaul",
        "linkedin": "https://www.linkedin.com/in/Samranjithpaul",
        "year": 2025,
        "version": "1.0.0"
    }))
}
