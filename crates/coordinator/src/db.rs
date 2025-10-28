// @author 
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use serde_json::Value;
use uuid::Uuid;

pub async fn make_pool() -> Result<PgPool> {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set, e.g. postgres://postgres:postgres@db:5432/jobe");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await?;
    sqlx::query_scalar::<_, i64>("SELECT 1").fetch_one(&pool).await?;
    Ok(pool)
}

pub async fn insert_job(pool: &PgPool, name: &str, dag: &Value) -> Result<Uuid> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO jobs (name, dag)
        VALUES ($1, $2)
        RETURNING id
        "#,
        name,
        dag
    )
    .fetch_one(pool)
    .await?;
    Ok(rec.id)
}