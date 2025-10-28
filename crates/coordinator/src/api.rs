use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[derive(Serialize)]
pub struct Health {
    ok: bool,
    db: bool,
}

pub async fn health(State(state): State<AppState>) -> Json<Health> {
    let db_ok = sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .is_ok();
    Json(Health {
        ok: db_ok,
        db: db_ok,
    })
}

#[derive(Deserialize)]
pub struct JobRequest {
    pub name: String,
    pub dag: Value,
}

#[derive(Serialize)]
pub struct JobResponse {
    pub id: Uuid,
    pub name: String,
}

pub async fn create_job(
    State(state): State<AppState>,
    Json(payload): Json<JobRequest>,
) -> Result<Json<JobResponse>, axum::http::StatusCode> {
    let id = crate::db::insert_job(&state.pool, &payload.name, &payload.dag)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(JobResponse {
        id,
        name: payload.name,
    }))
}

#[derive(Serialize, sqlx::FromRow)]
pub struct JobRecord {
    pub id: Uuid,
    pub name: String,
    pub dag: Value,
    pub status: String,
}

pub async fn get_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<JobRecord>, axum::http::StatusCode> {
    let rec = sqlx::query_as::<_, JobRecord>(
        r#"SELECT id, name, dag, status FROM jobs WHERE id = $1"#
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    Ok(Json(rec))
}
