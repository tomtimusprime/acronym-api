use crate::db::{acronym_by_id, all_acronyms, Acronym};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{extract, Extension, Json, Router, response::Redirect};
use serde::{Serialize, Deserialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use log::{error, info};
use sqlx::MySqlPool;

impl crate::db::Acronym {
    pub fn new() -> Acronym {
        Default::default()
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_all_acronyms_handler,
    ),
    components(
        schemas(crate::db::Acronym)
    ),
    tags(
        (name = "Acronym Api", description = "This is an API that manages definitions for common tech related acronyms.")
    )
)]
struct ApiDoc;

#[derive(Debug)]
pub struct ApiError {
    cause: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let message = format!("Error: {}", self.cause);
        (StatusCode::SERVICE_UNAVAILABLE, message).into_response()
    }
}

pub fn acronym_service() -> Router {
    Router::new()
        .route("/", get(get_all_acronyms_handler))
        .route("/:id", get(get_acronym_by_id_handler))
        .route("/add", post(add_acronym_handler))
        .route("/update", put(update_acronym_handler))
        .route("/delete/:id", delete(delete_acronym_handler))
        .route("/search/:term", get(search_acronym_handler))
        //.route(SwaggerUi::new("/swagger-ui/").url("/api-docs", ApiDoc::openapi()))
}

#[utoipa::path(
    get,
    path = "/",
    responses((status = 200, description = "Successful Response", body = crate::db::Acronym)),
)]
pub async fn get_all_acronyms_handler(
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

pub async fn get_acronym_by_id_handler(
    Extension(conn): Extension<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<Acronym>, StatusCode> {
    match acronym_by_id(&conn, id).await {
        Ok(acronym) => {
            info!("Acronym added successfully. Id: {:?}", id);
            Ok(Json(acronym))
        }
        Err(e) => {
            error!("Failed to get acronym: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

pub async fn add_acronym_handler(
    Extension(conn): Extension<MySqlPool>,
    extract::Json(acronym): extract::Json<Acronym>,
) -> Result<Json<i32>, StatusCode> {
    match crate::db::add_acronym(&conn, acronym.acronym, acronym.definition).await {
        Ok(id) => {
            info!("Added acronym successfully.");
            Ok(Json(id))
        }
        Err(e) => {
            error!("Failed to add acronym: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

pub async fn update_acronym_handler(
    Extension(conn): Extension<MySqlPool>,
    extract::Json(acronym): extract::Json<Acronym>,
) -> Result<StatusCode, ApiError> {
    match crate::db::update_acronym(&conn, &acronym).await {
        Ok(_) => {
            info!("Acronym updated succesfully. {:?}", &acronym.id);
            Ok(StatusCode::OK)
        }
        Err(e) => {
            error!("Failed to update acronym. {:?}", e);
            Err(ApiError {
                cause: format!("{:?}", e),
            })
        }
    }
}

pub async fn delete_acronym_handler(
    Extension(conn): Extension<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    match crate::db::delete_acronym(&conn, id).await {
        Ok(_) => {
            info!("Acronym deleted successfully. Id:{:?}", id);
            Ok(StatusCode::OK)
        }
        Err(e) => {
            error!("Failed to delete acronym. {:?}", e);
            Err(ApiError {
                cause: format!("{:?}", e),
            })
        }
    }
}

pub async fn search_acronym_handler(
    Extension(conn): Extension<MySqlPool>,
    Path(search_term): Path<String>,
) -> Result<Json<Vec<Acronym>>, StatusCode> {
    match crate::db::search_acronyms(&conn, &search_term).await {
        Ok(acronyms) => {
            info!("Acronyms successfully searched.");
            Ok(Json(acronyms))
        },
        Err(e) => {
            error!("Failed to search for acronyms: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

mod test {
    use super::*;
    use axum_test_helper::TestClient;

    async fn setup_connection() -> TestClient {
        dotenv::dotenv().ok();
        let connection_pool = crate::init_db().await.unwrap();
        let app = Router::new()
            .nest_service("/acronyms", crate::rest::acronym_service())
            .layer(Extension(connection_pool));
        TestClient::new(app)
    }

    #[tokio::test]
    async fn test_get_all_acronyms() {
        let client = setup_connection().await;
        let res = client.get("/acronyms").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        let acronyms: Vec<Acronym> = res.json().await;
        assert!(!acronyms.is_empty());
    }

    #[tokio::test]
    async fn test_get_acronym_by_id() {
        let client = setup_connection().await;
        let res = client.get("/acronyms/1").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        let acronym: Acronym = res.json().await;
        assert_eq!(acronym.id, 1)
    }

    #[tokio::test]
    async fn test_add_acronym() {
        let client = setup_connection().await;
        let new_acronym = Acronym {
            id: 45,
            acronym: "Test Acronym".to_string(),
            definition: "Test definition".to_string(),
        };
        let res = client.post("/acronyms/add").json(&new_acronym).send().await;
        println!("Response Status: {:?}", res.status());
        assert_eq!(res.status(), StatusCode::OK);
        let new_id: i32 = res.json().await;
        assert!(new_id > 0);
    }
}
