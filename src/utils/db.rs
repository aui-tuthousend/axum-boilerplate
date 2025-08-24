use std::env;
use sqlx::{postgres::PgPool, Pool, Postgres};

pub async fn init() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set"));
    
    PgPool::connect(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Failed to connect to database"))
}