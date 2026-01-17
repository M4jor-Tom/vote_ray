use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(ToSchema, Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
    email: String,
}

#[derive(ToSchema, Serialize)]
struct UserResponse {
    user: User,
    message: String,
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = u32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found successfully", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    tag = "users"
)]
async fn get_user(Path(id): Path<u32>) -> impl IntoResponse {
    if id == 1 {
        let user: User = User {
            id,
            username: "john_doe".to_string(),
            email: "john@example.com".to_string(),
        };
        (
            StatusCode::OK,
            Json(UserResponse {
                user,
                message: "User retrieved successfully".to_string(),
            }),
        )
            .into_response()
    } else {
        (StatusCode::NOT_FOUND, "User not found").into_response()
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_user,
    ),
    components(
        schemas(User, UserResponse)
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let doc = ApiDoc::openapi();
    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/users/:id", get(get_user))
        .merge(swagger_ui);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
