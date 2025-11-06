// ================================================================
//  Employee Management System (EMS)
//  Developed by: Sam Ranjith Paul
//  GitHub: https://github.com/samranjithpaul
//  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
//  Unauthorized removal of this header is prohibited.
// ================================================================

use actix_web::{web, HttpResponse, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::db::DbPool;
use crate::models::*;

// JWT Secret - should be in .env in production
const JWT_SECRET: &str = "your-secret-key-change-in-production";

pub fn create_token(admin_id: i32, email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        admin_id,
        email: email.to_string(),
        exp: (chrono::Utc::now().timestamp() + 86400) as usize, // 24 hours
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

pub async fn register(
    pool: web::Data<DbPool>,
    admin_data: web::Json<AdminRegister>,
) -> Result<HttpResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Database connection failed".to_string(),
            )));
        }
    };

    // Check if admin already exists using raw SQL
    #[derive(QueryableByName)]
    struct AdminId {
        #[diesel(sql_type = Integer)]
        admin_id: i32,
    }
    
    let existing: Result<AdminId, _> = diesel::sql_query(
        "SELECT admin_id FROM admins WHERE email = $1"
    )
    .bind::<Text, _>(&admin_data.email)
    .get_result(&mut conn);

    if existing.is_ok() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "Admin with this email already exists".to_string(),
        )));
    }

    // Hash password
    let password_hash = match hash(&admin_data.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Password hashing error: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to hash password".to_string(),
            )));
        }
    };

    // Insert new admin using raw SQL
    #[derive(QueryableByName)]
    struct NewAdminId {
        #[diesel(sql_type = Integer)]
        admin_id: i32,
    }
    
    let result: Result<NewAdminId, _> = diesel::sql_query(
        "INSERT INTO admins (email, password_hash) VALUES ($1, $2) RETURNING admin_id"
    )
    .bind::<Text, _>(&admin_data.email)
    .bind::<Text, _>(&password_hash)
    .get_result(&mut conn);

    match result {
        Ok(new_admin) => {
            let token = match create_token(new_admin.admin_id, &admin_data.email) {
                Ok(token) => token,
                Err(e) => {
                    eprintln!("Token creation error: {}", e);
                    return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                        "Failed to create token".to_string(),
                    )));
                }
            };
            Ok(HttpResponse::Ok().json(LoginResponse {
                token,
                message: "Admin registered successfully".to_string(),
            }))
        }
        Err(e) => {
            eprintln!("Registration error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to register admin".to_string(),
            )))
        }
    }
}

pub async fn login(
    pool: web::Data<DbPool>,
    login_data: web::Json<AdminLogin>,
) -> Result<HttpResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Database connection failed".to_string(),
            )));
        }
    };

    // Get admin from database using raw SQL
    #[derive(QueryableByName)]
    struct AdminQuery {
        #[diesel(sql_type = Integer)]
        admin_id: i32,
        #[diesel(sql_type = Text)]
        password_hash: String,
    }

    let result: Result<AdminQuery, _> = diesel::sql_query(
        "SELECT admin_id, password_hash FROM admins WHERE email = $1"
    )
    .bind::<Text, _>(&login_data.email)
    .get_result(&mut conn);

    match result {
        Ok(admin) => {
            // Verify password
            if verify(&login_data.password, &admin.password_hash).unwrap_or(false) {
                let token = match create_token(admin.admin_id, &login_data.email) {
                    Ok(token) => token,
                    Err(e) => {
                        eprintln!("Token creation error: {}", e);
                        return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                            "Failed to create token".to_string(),
                        )));
                    }
                };
                Ok(HttpResponse::Ok().json(LoginResponse {
                    token,
                    message: "Login successful".to_string(),
                }))
            } else {
                Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                    "Invalid email or password".to_string(),
                )))
            }
        }
        Err(_) => {
            Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "Invalid email or password".to_string(),
            )))
        }
    }
}

