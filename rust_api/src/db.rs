// ================================================================
//  Employee Management System (EMS)
//  Developed by: Sam Ranjith Paul
//  GitHub: https://github.com/samranjithpaul
//  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
//  Unauthorized removal of this header is prohibited.
// ================================================================

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            eprintln!("ERROR: DATABASE_URL environment variable is not set!");
            std::process::exit(1);
        });

    eprintln!("Connecting to database: {}", database_url.split('@').last().unwrap_or("***"));

    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    
    Pool::builder()
        .build(manager)
        .unwrap_or_else(|e| {
            eprintln!("ERROR: Failed to create database pool: {}", e);
            eprintln!("Database URL: {}", database_url.split('@').last().unwrap_or("***"));
            std::process::exit(1);
        })
}

