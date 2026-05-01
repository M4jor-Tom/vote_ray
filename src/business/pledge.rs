use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pledge {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl Pledge {
    pub fn new(id: Uuid, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}
