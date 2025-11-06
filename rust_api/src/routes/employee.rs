// ================================================================
//  Employee Management System (EMS)
//  Developed by: Sam Ranjith Paul
//  GitHub: https://github.com/samranjithpaul
//  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
//  Unauthorized removal of this header is prohibited.
// ================================================================

use actix_web::{web, HttpResponse};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text, Nullable, Date, Timestamp};
use serde_json::json;

use crate::db::DbPool;
use crate::models::*;
use crate::routes::create_audit_log;

#[derive(QueryableByName)]
struct EmployeeRow {
    #[diesel(sql_type = Integer)]
    emp_id: i32,
    #[diesel(sql_type = Text)]
    first_name: String,
    #[diesel(sql_type = Text)]
    last_name: String,
    #[diesel(sql_type = Nullable<Text>)]
    email: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    phone: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    department: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    designation: Option<String>,
    #[diesel(sql_type = Nullable<Date>)]
    joining_date: Option<NaiveDate>,
    #[diesel(sql_type = Text)]
    status: String,
    #[diesel(sql_type = Timestamp)]
    created_at: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    updated_at: NaiveDateTime,
}

impl From<EmployeeRow> for Employee {
    fn from(row: EmployeeRow) -> Self {
        Employee {
            emp_id: row.emp_id,
            first_name: row.first_name,
            last_name: row.last_name,
            email: row.email,
            phone: row.phone,
            department: row.department,
            designation: row.designation,
            joining_date: row.joining_date,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

pub async fn get_employees(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Database connection failed".to_string(),
            ));
        }
    };

    let result: Result<Vec<EmployeeRow>, _> = diesel::sql_query(
        "SELECT emp_id, first_name, last_name, email, phone, department, designation, joining_date, status, created_at, updated_at FROM employees ORDER BY emp_id"
    )
    .load(&mut conn);

    match result {
        Ok(rows) => {
            let employees: Vec<Employee> = rows.into_iter().map(Employee::from).collect();
            HttpResponse::Ok().json(ApiResponse::success(
                "Employees retrieved successfully".to_string(),
                employees,
            ))
        }
        Err(e) => {
            eprintln!("Error fetching employees: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to fetch employees".to_string(),
            ))
        }
    }
}

pub async fn get_employee(
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

    let result: Result<EmployeeRow, _> = diesel::sql_query(
        "SELECT emp_id, first_name, last_name, email, phone, department, designation, joining_date, status, created_at, updated_at FROM employees WHERE emp_id = $1"
    )
    .bind::<Integer, _>(&emp_id)
    .get_result(&mut conn);

    match result {
        Ok(row) => {
            let employee = Employee::from(row);
            HttpResponse::Ok().json(ApiResponse::success(
                "Employee retrieved successfully".to_string(),
                employee,
            ))
        }
        Err(_) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error(
                "Employee not found".to_string(),
            ))
        }
    }
}

pub async fn create_employee(
    pool: web::Data<DbPool>,
    employee_data: web::Json<EmployeeCreate>,
    admin_id: i32,
) -> HttpResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Database connection failed".to_string(),
            ));
        }
    };

    let joining_date = employee_data.joining_date.as_ref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
    let status = employee_data.status.as_deref().unwrap_or("Active");

    #[derive(QueryableByName)]
    struct EmpId {
        #[diesel(sql_type = Integer)]
        emp_id: i32,
    }

    let result: Result<EmpId, _> = diesel::sql_query(
        "INSERT INTO employees (first_name, last_name, email, phone, department, designation, joining_date, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING emp_id"
    )
    .bind::<Text, _>(&employee_data.first_name)
    .bind::<Text, _>(&employee_data.last_name)
    .bind::<Nullable<Text>, _>(&employee_data.email)
    .bind::<Nullable<Text>, _>(&employee_data.phone)
    .bind::<Nullable<Text>, _>(&employee_data.department)
    .bind::<Nullable<Text>, _>(&employee_data.designation)
    .bind::<Nullable<Date>, _>(&joining_date)
    .bind::<Text, _>(&status)
    .get_result(&mut conn);

    match result {
        Ok(emp) => {
            let emp_id = emp.emp_id;
            // Create audit log
            let _ = create_audit_log(
                &mut conn,
                admin_id,
                Some(emp_id),
                "CREATE_EMPLOYEE",
                Some(&format!("Created employee: {} {}", employee_data.first_name, employee_data.last_name)),
            );

            HttpResponse::Ok().json(ApiResponse::success(
                "Employee created successfully".to_string(),
                json!({"emp_id": emp_id}),
            ))
        }
        Err(e) => {
            eprintln!("Error creating employee: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to create employee".to_string(),
            ))
        }
    }
}

pub async fn update_employee(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    employee_data: web::Json<EmployeeUpdate>,
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

    // First, get the current employee data
    let current: Result<EmployeeRow, _> = diesel::sql_query(
        "SELECT emp_id, first_name, last_name, email, phone, department, designation, joining_date, status, created_at, updated_at FROM employees WHERE emp_id = $1"
    )
    .bind::<Integer, _>(&emp_id)
    .get_result(&mut conn);

    let current_employee = match current {
        Ok(emp) => emp,
        Err(_) => {
            return HttpResponse::NotFound().json(ApiResponse::<()>::error(
                "Employee not found".to_string(),
            ));
        }
    };

    // Use current values if update values are None
    let first_name = employee_data.first_name.as_ref().unwrap_or(&current_employee.first_name);
    let last_name = employee_data.last_name.as_ref().unwrap_or(&current_employee.last_name);
    let email = employee_data.email.as_ref().or(current_employee.email.as_ref());
    let phone = employee_data.phone.as_ref().or(current_employee.phone.as_ref());
    let department = employee_data.department.as_ref().or(current_employee.department.as_ref());
    let designation = employee_data.designation.as_ref().or(current_employee.designation.as_ref());
    let status = employee_data.status.as_ref().unwrap_or(&current_employee.status);
    
    let joining_date = employee_data.joining_date.as_ref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .or(current_employee.joining_date);

    // Update employee
    let result = diesel::sql_query(
        "UPDATE employees SET first_name = $1, last_name = $2, email = $3, phone = $4, department = $5, designation = $6, joining_date = $7, status = $8, updated_at = NOW() WHERE emp_id = $9"
    )
    .bind::<Text, _>(first_name)
    .bind::<Text, _>(last_name)
    .bind::<Nullable<Text>, _>(email)
    .bind::<Nullable<Text>, _>(phone)
    .bind::<Nullable<Text>, _>(department)
    .bind::<Nullable<Text>, _>(designation)
    .bind::<Nullable<Date>, _>(&joining_date)
    .bind::<Text, _>(status)
    .bind::<Integer, _>(&emp_id)
    .execute(&mut conn);

    match result {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                // Create audit log
                let _ = create_audit_log(
                    &mut conn,
                    admin_id,
                    Some(emp_id),
                    "UPDATE_EMPLOYEE",
                    Some(&format!("Updated employee ID: {}", emp_id)),
                );

                HttpResponse::Ok().json(ApiResponse::success(
                    "Employee updated successfully".to_string(),
                    json!({"emp_id": emp_id}),
                ))
            } else {
                HttpResponse::NotFound().json(ApiResponse::<()>::error(
                    "Employee not found".to_string(),
                ))
            }
        }
        Err(e) => {
            eprintln!("Error updating employee: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to update employee".to_string(),
            ))
        }
    }
}

pub async fn delete_employee(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
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

    // Get employee name before deletion for audit log
    #[derive(QueryableByName)]
    struct EmployeeName {
        #[diesel(sql_type = Text)]
        name: String,
    }
    
    let employee_name: Result<EmployeeName, _> = diesel::sql_query(
        "SELECT first_name || ' ' || last_name as name FROM employees WHERE emp_id = $1"
    )
    .bind::<Integer, _>(&emp_id)
    .get_result(&mut conn);

    let result = diesel::sql_query(
        "DELETE FROM employees WHERE emp_id = $1"
    )
    .bind::<Integer, _>(&emp_id)
    .execute(&mut conn);

    match result {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                // Create audit log
                let details = employee_name
                    .map(|name| format!("Deleted employee: {}", name.name))
                    .ok();
                let _ = create_audit_log(
                    &mut conn,
                    admin_id,
                    Some(emp_id),
                    "DELETE_EMPLOYEE",
                    details.as_deref(),
                );

                HttpResponse::Ok().json(ApiResponse::success(
                    "Employee deleted successfully".to_string(),
                    json!({"emp_id": emp_id}),
                ))
            } else {
                HttpResponse::NotFound().json(ApiResponse::<()>::error(
                    "Employee not found".to_string(),
                ))
            }
        }
        Err(e) => {
            eprintln!("Error deleting employee: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to delete employee".to_string(),
            ))
        }
    }
}

