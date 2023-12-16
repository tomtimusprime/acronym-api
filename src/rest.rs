use crate::db::{acronym_by_id, all_acronyms, Acronym};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{extract, Extension, Json, Router};
use log::{error, info};
use sqlx::MySqlPool;

pub fn acronym_service() -> Router {
    Router::new()
        .route("/", get(get_all_acronyms_handler))
        .route("/:id", get(get_acronym_by_id_handler))
        .route("/add", post(add_acronym_handler))
        .route("/update", put(update_acronym_handler))
        .route("/delete/:id", delete(delete_acronym_handler))
}

#[derive(Debug)]
struct ApiError {
    cause: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let message = format!("Error: {}", self.cause);
        (StatusCode::SERVICE_UNAVAILABLE, message).into_response()
    }
}

async fn get_all_acronyms_handler(
    Extension(conn): Extension<MySqlPool>,
) -> Result<Json<Vec<Acronym>>, StatusCode> {
    match all_acronyms(&conn).await {
        Ok(acronyms) => {
            info!("Retrieved Acronyms Successfully!");
            Ok(Json(acronyms))
        }
        Err(e) => {
            error!("Failed to get acronyms {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn get_acronym_by_id_handler(
    Extension(conn): Extension<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<Acronym>, StatusCode> {
    if let Ok(acronym) = acronym_by_id(&conn, id).await {
        Ok(Json(acronym))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn add_acronym_handler(
    Extension(conn): Extension<MySqlPool>,
    extract::Json(acronym): extract::Json<Acronym>,
) -> Result<Json<i32>, StatusCode> {
    if let Ok(new_id) = crate::db::add_acronym(&conn, acronym.acronym, acronym.definition).await {
        Ok(Json(new_id))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn update_acronym_handler(
    Extension(conn): Extension<MySqlPool>,
    extract::Json(acronym): extract::Json<Acronym>,
) -> Result<StatusCode, ApiError> {
    match crate::db::update_acronym(&conn, &acronym).await {
        Ok(_) => {
            info!("Acronym update succesfully.");
            Ok(StatusCode::OK)
        }
        Err(e) => {
            error!("Failed to update acronym {:?}", e);
            Err(ApiError {cause: format!("{:?}", e)})
        }
    }
}

async fn delete_acronym_handler(
    Extension(conn): Extension<MySqlPool>,
    Path(id): Path<i32>,
) -> StatusCode {
    if crate::db::delete_acronym(&conn, id).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}
