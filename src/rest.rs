use crate::db::{all_acronyms, acronym_by_id, Acronym};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{extract, Extension, Json, Router};
use sqlx::MySqlPool;

pub fn acronym_service() -> Router {
    Router::new()
    .route("/", get(get_all_acronyms_handler))
    .route("/:id", get(get_acronym_by_id_handler))
    .route("/add", post(add_acronym_handler))
}

async fn get_all_acronyms_handler(Extension(conn): Extension<MySqlPool>) -> Result<Json<Vec<Acronym>>, StatusCode> {
    if let Ok(acronyms) = all_acronyms(&conn).await {
        Ok(Json(acronyms))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn get_acronym_by_id_handler(Extension(conn):Extension<MySqlPool>, Path(id): Path<i32>) -> Result<Json<Acronym>, StatusCode>{
    if let Ok(acronym) = acronym_by_id(&conn, id).await {
        Ok(Json(acronym))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn add_acronym_handler(Extension(conn):Extension<MySqlPool>, extract::Json(acronym): extract::Json<Acronym>) -> Result<Json<i32>, StatusCode> {
    if let Ok(new_id) = crate::db::add_acronym(&conn, acronym.acronym, acronym.definition).await {
        Ok(Json(new_id))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}