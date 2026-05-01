use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use uuid::Uuid;

use crate::business::candidate::Candidate;
use crate::database::candidate_repository::{CandidateRepository, CandidateRepositoryError};
use crate::rest::dto::candidate::{
    CandidateResponse, CreateCandidateRequest, UpdateCandidateRequest,
};

pub struct CandidateHandler {
    repository: Arc<CandidateRepository>,
}

impl CandidateHandler {
    pub fn new(repository: Arc<CandidateRepository>) -> Self {
        Self { repository }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/candidates",
    request_body = CreateCandidateRequest,
    responses(
        (status = 201, description = "Candidate created successfully", body = CandidateResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "candidates"
)]
pub async fn create_candidate(
    State(handler): State<Arc<CandidateHandler>>,
    Json(request): Json<CreateCandidateRequest>,
) -> impl IntoResponse {
    let candidate = Candidate::new(request.id, request.name);

    match handler.repository.create(&candidate).await {
        Ok(created_candidate) => (
            StatusCode::CREATED,
            Json(CandidateResponse::from(created_candidate)),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create candidate",
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/candidates/{id}",
    params(
        ("id" = Uuid, Path, description = "Candidate ID")
    ),
    responses(
        (status = 200, description = "Candidate retrieved successfully", body = CandidateResponse),
        (status = 404, description = "Candidate not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "candidates"
)]
pub async fn get_candidate(
    State(handler): State<Arc<CandidateHandler>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match handler.repository.get_by_id(&id).await {
        Ok(candidate) => (StatusCode::OK, Json(CandidateResponse::from(candidate))).into_response(),
        Err(CandidateRepositoryError::NotFound) => {
            (StatusCode::NOT_FOUND, "Candidate not found").into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve candidate",
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/candidates",
    responses(
        (status = 200, description = "Candidates retrieved successfully", body = Vec<CandidateResponse>),
        (status = 500, description = "Internal server error")
    ),
    tag = "candidates"
)]
pub async fn get_all_candidates(State(handler): State<Arc<CandidateHandler>>) -> impl IntoResponse {
    match handler.repository.get_all().await {
        Ok(candidates) => {
            let responses: Vec<CandidateResponse> = candidates
                .into_iter()
                .map(CandidateResponse::from)
                .collect();
            (StatusCode::OK, Json(responses)).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve candidates",
        )
            .into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/candidates/{id}",
    params(
        ("id" = Uuid, Path, description = "Candidate ID")
    ),
    request_body = UpdateCandidateRequest,
    responses(
        (status = 200, description = "Candidate updated successfully", body = CandidateResponse),
        (status = 404, description = "Candidate not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "candidates"
)]
pub async fn update_candidate(
    State(handler): State<Arc<CandidateHandler>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateCandidateRequest>,
) -> impl IntoResponse {
    let candidate = Candidate::new(id, request.name);

    match handler.repository.update(&candidate).await {
        Ok(updated_candidate) => (
            StatusCode::OK,
            Json(CandidateResponse::from(updated_candidate)),
        )
            .into_response(),
        Err(CandidateRepositoryError::NotFound) => {
            (StatusCode::NOT_FOUND, "Candidate not found").into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update candidate",
        )
            .into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/candidates/{id}",
    params(
        ("id" = Uuid, Path, description = "Candidate ID")
    ),
    responses(
        (status = 204, description = "Candidate deleted successfully"),
        (status = 404, description = "Candidate not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "candidates"
)]
pub async fn delete_candidate(
    State(handler): State<Arc<CandidateHandler>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match handler.repository.delete(&id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, "Candidate not found").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete candidate",
        )
            .into_response(),
    }
}
