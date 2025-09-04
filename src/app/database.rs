use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn database_init() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error building a connection pool.");
    Ok(pool)
}
