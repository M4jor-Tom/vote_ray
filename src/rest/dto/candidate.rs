use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCandidateRequest {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateCandidateRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CandidateResponse {
    pub id: Uuid,
    pub name: String,
}

impl From<crate::business::candidate::Candidate> for CandidateResponse {
    fn from(candidate: crate::business::candidate::Candidate) -> Self {
        Self {
            id: candidate.id,
            name: candidate.name,
        }
    }
}
