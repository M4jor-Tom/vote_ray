use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePledgeRequest {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePledgeRequest {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PledgeResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl From<crate::business::pledge::Pledge> for PledgeResponse {
    fn from(pledge: crate::business::pledge::Pledge) -> Self {
        Self {
            id: pledge.id,
            name: pledge.name,
            description: pledge.description,
        }
    }
}
