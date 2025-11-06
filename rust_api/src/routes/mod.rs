// ================================================================
//  Employee Management System (EMS)
//  Developed by: Sam Ranjith Paul
//  GitHub: https://github.com/samranjithpaul
//  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
//  Unauthorized removal of this header is prohibited.
// ================================================================

pub mod auth;
pub mod employee;
pub mod history;
pub mod payslip;

use actix_web::web;
use crate::db::DbPool;
use crate::models::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;

// Helper function to create audit log
pub fn create_audit_log(
    conn: &mut diesel::PgConnection,
    admin_id: i32,
    emp_id: Option<i32>,
    action: &str,
    details: Option<&str>,
) -> Result<(), diesel::result::Error> {
    diesel::sql_query(
        "INSERT INTO audit_logs (admin_id, emp_id, action, details) VALUES ($1, $2, $3, $4)"
    )
    .bind::<diesel::sql_types::Integer, _>(&admin_id)
    .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(&emp_id)
    .bind::<diesel::sql_types::Text, _>(&action)
    .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(&details)
    .execute(conn)?;
    Ok(())
}

