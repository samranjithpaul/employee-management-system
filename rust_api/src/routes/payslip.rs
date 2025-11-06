// ================================================================
//  Employee Management System (EMS)
//  Developed by: Sam Ranjith Paul
//  GitHub: https://github.com/samranjithpaul
//  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
//  Unauthorized removal of this header is prohibited.
// ================================================================

use actix_web::{web, HttpResponse, Result};
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::ApiResponse;
use crate::routes::create_audit_log;

pub async fn upload_payslip(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    mut payload: Multipart,
    admin_id: i32,
) -> Result<HttpResponse> {
    let emp_id = path.into_inner();
    
    // Create directory for employee payslips
    let upload_dir = format!("./uploads/payslips/{}", emp_id);
    fs::create_dir_all(&upload_dir).map_err(|e| {
        eprintln!("Error creating directory: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to create upload directory")
    })?;

    let mut filename = None;
    let mut file_data = Vec::new();

    // Process multipart form data
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        
        if let Some(name) = content_disposition.get_name() {
            if name == "file" {
                if let Some(original_filename) = content_disposition.get_filename() {
                    // Get file extension and validate it's a supported format
                    let file_extension = Path::new(original_filename)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.to_lowercase())
                        .unwrap_or_else(|| "pdf".to_string());
                    
                    // Validate file type (PDF, Excel, Word)
                    let allowed_extensions = ["pdf", "xlsx", "xls", "docx", "doc"];
                    if !allowed_extensions.contains(&file_extension.as_str()) {
                        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                            format!("Unsupported file type. Allowed: PDF, Excel (.xlsx, .xls), Word (.docx, .doc)").to_string(),
                        )));
                    }
                    
                    let unique_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
                    filename = Some(unique_filename.clone());
                    
                    // Read file data
                    while let Some(chunk) = field.try_next().await? {
                        file_data.extend_from_slice(&chunk);
                    }
                }
            }
        }
    }

    if let Some(ref fname) = filename {
        let file_path = format!("{}/{}", upload_dir, fname);
        
        // Write file to disk
        let mut file = fs::File::create(&file_path).map_err(|e| {
            eprintln!("Error creating file: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to create file")
        })?;
        
        file.write_all(&file_data).map_err(|e| {
            eprintln!("Error writing file: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to write file")
        })?;

        // Update database with file path
        let mut conn = pool.get().map_err(|e| {
            eprintln!("Database connection error: {}", e);
            actix_web::error::ErrorInternalServerError("Database connection failed")
        })?;

        // Store file path in employment_history or create a separate record
        // For simplicity, we'll store it in a new table or update existing history
        // Here we'll just log it in audit and return success
        
        // Create audit log
        let _ = create_audit_log(
            &mut conn,
            admin_id,
            Some(emp_id),
            "UPLOAD_PAYSLIP",
            Some(&format!("Uploaded payslip: {}", fname)),
        );

        Ok(HttpResponse::Ok().json(ApiResponse::success(
            "Payslip uploaded successfully".to_string(),
            serde_json::json!({
                "filename": fname,
                "path": file_path,
                "size": file_data.len()
            }),
        )))
    } else {
        Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "No file provided".to_string(),
        )))
    }
}

pub async fn get_payslip(
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let filename = path.into_inner();
    
    // Find file in payslips directory
    let payslip_dir = "./uploads/payslips";
    let file_path = format!("{}/{}", payslip_dir, filename);
    
    // Search in all subdirectories
    let mut found_path = None;
    if let Ok(entries) = fs::read_dir(payslip_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let potential_path = entry.path().join(&filename);
                if potential_path.exists() {
                    found_path = Some(potential_path);
                    break;
                }
            }
        }
    }
    
    if let Some(path) = found_path {
        let file_data = fs::read(&path).map_err(|e| {
            eprintln!("Error reading file: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to read file")
        })?;
        
        // Determine content type based on file extension
        let content_type = if let Some(ext) = Path::new(&filename).extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "pdf" => "application/pdf",
                "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                "xls" => "application/vnd.ms-excel",
                "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                "doc" => "application/msword",
                _ => "application/octet-stream",
            }
        } else {
            "application/octet-stream"
        };
        
        Ok(HttpResponse::Ok()
            .content_type(content_type)
            .body(file_data))
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "Payslip not found".to_string(),
        )))
    }
}

pub async fn list_payslips(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> HttpResponse {
    let emp_id = path.into_inner();
    let payslip_dir = format!("./uploads/payslips/{}", emp_id);
    
    let mut payslips = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&payslip_dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        payslips.push(serde_json::json!({
                            "filename": file_name,
                            "size": metadata.len(),
                            "path": format!("/payslip/{}", file_name)
                        }));
                    }
                }
            }
        }
    }
    
    HttpResponse::Ok().json(crate::models::ApiResponse::success(
        "Payslips retrieved successfully".to_string(),
        payslips,
    ))
}

