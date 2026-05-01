use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Candidate {
    pub id: Uuid,
    pub name: String,
}

impl Candidate {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}
