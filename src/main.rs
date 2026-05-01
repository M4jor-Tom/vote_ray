use axum::{routing::get, Router};
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod business;
mod database;
mod rest;

use database::{
    create_pool, CandidateRepository, DatabaseManager, PledgeRepository,
};
use rest::v1;

#[derive(OpenApi)]
#[openapi(
    paths(
        rest::handlers::candidate::create_candidate,
        rest::handlers::candidate::get_candidate,
        rest::handlers::candidate::get_all_candidates,
        rest::handlers::candidate::update_candidate,
        rest::handlers::candidate::delete_candidate,
        rest::handlers::pledge::create_pledge,
        rest::handlers::pledge::get_pledge,
        rest::handlers::pledge::get_all_pledges,
        rest::handlers::pledge::update_pledge,
        rest::handlers::pledge::delete_pledge,
    ),
    components(
        schemas(
            rest::dto::candidate::CreateCandidateRequest,
            rest::dto::candidate::UpdateCandidateRequest,
            rest::dto::candidate::CandidateResponse,
            rest::dto::pledge::CreatePledgeRequest,
            rest::dto::pledge::UpdatePledgeRequest,
            rest::dto::pledge::PledgeResponse,
        )
    ),
    tags(
        (name = "candidates", description = "Candidate management endpoints"),
        (name = "pledges", description = "Pledge management endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    dotenvy::dotenv().ok();

    let db_manager = Arc::new(DatabaseManager::new("vote_ray_postgres"));
    
    // Start the database container
    if let Err(e) = db_manager.start_database().await {
        error!("Failed to start database: {}", e);
        std::process::exit(1);
    }

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://vote_ray:your_password@localhost:5432/vote_ray".to_string());

    // Wait a moment for database to be fully ready
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let pool = create_pool(&database_url)
        .await
        .expect("Failed to connect to database");

    let candidate_repository = Arc::new(CandidateRepository::new(pool.clone()));
    let pledge_repository = Arc::new(PledgeRepository::new(pool));

    let doc = ApiDoc::openapi();
    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc);

    let v1_router = v1::create_v1_router(candidate_repository, pledge_repository);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/api/v1", v1_router)
        .merge(swagger_ui);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server starting on http://0.0.0.0:3000");

    // Set up graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(db_manager))
        .await
        .unwrap();
}

async fn shutdown_signal(db_manager: Arc<DatabaseManager>) {
    signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    
    info!("Shutting down gracefully...");
    
    if let Err(e) = db_manager.cleanup_database().await {
        error!("Failed to cleanup database: {}", e);
    } else {
        info!("Database cleanup completed");
    }
}
