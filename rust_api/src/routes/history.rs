// ================================================================
//  Employee Management System (EMS)
//  Developed by: Sam Ranjith Paul
//  GitHub: https://github.com/samranjithpaul
//  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
//  Unauthorized removal of this header is prohibited.
// ================================================================

use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text, Nullable, Date, Timestamp};

use crate::db::DbPool;
use crate::models::*;
use crate::routes::create_audit_log;

#[derive(QueryableByName)]
struct EmploymentHistoryRow {
    #[diesel(sql_type = Integer)]
    history_id: i32,
    #[diesel(sql_type = Integer)]
    emp_id: i32,
    #[diesel(sql_type = Text)]
    company_name: String,
    #[diesel(sql_type = Nullable<Date>)]
    start_date: Option<NaiveDate>,
    #[diesel(sql_type = Nullable<Date>)]
    end_date: Option<NaiveDate>,
    #[diesel(sql_type = Nullable<Text>)]
    position: Option<String>,
    #[diesel(sql_type = Timestamp)]
    created_at: chrono::NaiveDateTime,
}

impl From<EmploymentHistoryRow> for EmploymentHistory {
    fn from(row: EmploymentHistoryRow) -> Self {
        EmploymentHistory {
            history_id: row.history_id,
            emp_id: row.emp_id,
            company_name: row.company_name,
            start_date: row.start_date,
            end_date: row.end_date,
            role: row.position, // Map position (DB) to role (API)
            payslip_pdf_path: None, // Not stored in employment_history table
            created_at: row.created_at,
        }
    }
}

pub async fn get_employment_history(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> HttpResponse {
    let emp_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Database connection failed".to_string(),
            ));
        }
    };

    let result: Result<Vec<EmploymentHistoryRow>, _> = diesel::sql_query(
        "SELECT history_id, emp_id, company_name, start_date, end_date, position, created_at FROM employment_history WHERE emp_id = $1 ORDER BY start_date DESC"
    )
    .bind::<Integer, _>(&emp_id)
    .load(&mut conn);

    match result {
        Ok(rows) => {
            let history: Vec<EmploymentHistory> = rows.into_iter().map(EmploymentHistory::from).collect();
            HttpResponse::Ok().json(ApiResponse::success(
                "Employment history retrieved successfully".to_string(),
                history,
            ))
        }
        Err(e) => {
            eprintln!("Error fetching employment history: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to fetch employment history".to_string(),
            ))
        }
    }
}

pub async fn add_employment_history(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    history_data: web::Json<EmploymentHistoryCreate>,
    admin_id: i32,
) -> HttpResponse {
    let emp_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Database connection failed".to_string(),
            ));
        }
    };

    let start_date = history_data.start_date.as_ref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
    let end_date = history_data.end_date.as_ref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    #[derive(QueryableByName)]
    struct HistoryId {
        #[diesel(sql_type = Integer)]
        history_id: i32,
    }

    let result: Result<HistoryId, _> = diesel::sql_query(
        "INSERT INTO employment_history (emp_id, company_name, start_date, end_date, position) VALUES ($1, $2, $3, $4, $5) RETURNING history_id"
    )
    .bind::<Integer, _>(&emp_id)
    .bind::<Text, _>(&history_data.company_name)
    .bind::<Nullable<Date>, _>(&start_date)
    .bind::<Nullable<Date>, _>(&end_date)
    .bind::<Nullable<Text>, _>(&history_data.role) // API uses 'role', DB uses 'position'
    .get_result(&mut conn);

    match result {
        Ok(history) => {
            let history_id = history.history_id;
            // Create audit log
            let _ = create_audit_log(
                &mut conn,
                admin_id,
                Some(emp_id),
                "ADD_EMPLOYMENT_HISTORY",
                Some(&format!("Added employment history: {}", history_data.company_name)),
            );

            HttpResponse::Ok().json(ApiResponse::success(
                "Employment history added successfully".to_string(),
                serde_json::json!({"history_id": history_id}),
            ))
        }
        Err(e) => {
            eprintln!("Error adding employment history: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to add employment history".to_string(),
            ))
        }
    }
}

