use crate::db::{all_acronyms, acronym_by_id, Acronym};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{extract, Extension, Json, Router};
use sqlx::MySqlPool;

pub fn acronym_service() -> Router {
    Router::new()
    .route("/", get(get_all_acronyms))
    .route("/:id", get(get_acronym_by_id))
}

async fn get_all_acronyms(Extension(conn): Extension<MySqlPool>) -> Result<Json<Vec<Acronym>>, StatusCode> {
    if let Ok(acronyms) = all_acronyms(&conn).await {
        Ok(Json(acronyms))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn get_acronym_by_id(Extension(conn):Extension<MySqlPool>, Path(id): Path<i32>) -> Result<Json<Acronym>, StatusCode>{
    if let Ok(acronym) = acronym_by_id(&conn, id).await {
        Ok(Json(acronym))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}