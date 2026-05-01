use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use uuid::Uuid;

use crate::business::pledge::Pledge;
use crate::database::pledge_repository::{PledgeRepository, PledgeRepositoryError};
use crate::rest::dto::pledge::{CreatePledgeRequest, PledgeResponse, UpdatePledgeRequest};

pub struct PledgeHandler {
    repository: Arc<PledgeRepository>,
}

impl PledgeHandler {
    pub fn new(repository: Arc<PledgeRepository>) -> Self {
        Self { repository }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/pledges",
    request_body = CreatePledgeRequest,
    responses(
        (status = 201, description = "Pledge created successfully", body = PledgeResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "pledges"
)]
pub async fn create_pledge(
    State(handler): State<Arc<PledgeHandler>>,
    Json(request): Json<CreatePledgeRequest>,
) -> impl IntoResponse {
    let pledge = Pledge::new(request.id, request.name, request.description);

    match handler.repository.create(&pledge).await {
        Ok(created_pledge) => (
            StatusCode::CREATED,
            Json(PledgeResponse::from(created_pledge)),
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create pledge").into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/pledges/{id}",
    params(
        ("id" = Uuid, Path, description = "Pledge ID")
    ),
    responses(
        (status = 200, description = "Pledge retrieved successfully", body = PledgeResponse),
        (status = 404, description = "Pledge not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "pledges"
)]
pub async fn get_pledge(
    State(handler): State<Arc<PledgeHandler>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match handler.repository.get_by_id(&id).await {
        Ok(pledge) => (StatusCode::OK, Json(PledgeResponse::from(pledge))).into_response(),
        Err(PledgeRepositoryError::NotFound) => {
            (StatusCode::NOT_FOUND, "Pledge not found").into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve pledge",
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/pledges",
    responses(
        (status = 200, description = "Pledges retrieved successfully", body = Vec<PledgeResponse>),
        (status = 500, description = "Internal server error")
    ),
    tag = "pledges"
)]
pub async fn get_all_pledges(State(handler): State<Arc<PledgeHandler>>) -> impl IntoResponse {
    match handler.repository.get_all().await {
        Ok(pledges) => {
            let responses: Vec<PledgeResponse> =
                pledges.into_iter().map(PledgeResponse::from).collect();
            (StatusCode::OK, Json(responses)).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve pledges",
        )
            .into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/pledges/{id}",
    params(
        ("id" = Uuid, Path, description = "Pledge ID")
    ),
    request_body = UpdatePledgeRequest,
    responses(
        (status = 200, description = "Pledge updated successfully", body = PledgeResponse),
        (status = 404, description = "Pledge not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "pledges"
)]
pub async fn update_pledge(
    State(handler): State<Arc<PledgeHandler>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdatePledgeRequest>,
) -> impl IntoResponse {
    let pledge = Pledge::new(id, request.name, request.description);

    match handler.repository.update(&pledge).await {
        Ok(updated_pledge) => {
            (StatusCode::OK, Json(PledgeResponse::from(updated_pledge))).into_response()
        }
        Err(PledgeRepositoryError::NotFound) => {
            (StatusCode::NOT_FOUND, "Pledge not found").into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update pledge").into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/pledges/{id}",
    params(
        ("id" = Uuid, Path, description = "Pledge ID")
    ),
    responses(
        (status = 204, description = "Pledge deleted successfully"),
        (status = 404, description = "Pledge not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "pledges"
)]
pub async fn delete_pledge(
    State(handler): State<Arc<PledgeHandler>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match handler.repository.delete(&id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, "Pledge not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete pledge").into_response(),
    }
}
