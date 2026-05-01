use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

use crate::business::pledge::Pledge;

#[derive(Debug, Clone)]
pub struct PledgeRepository {
    pool: Arc<PgPool>,
}

#[derive(Debug, thiserror::Error)]
pub enum PledgeRepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Pledge not found")]
    NotFound,
}

impl PledgeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    pub async fn create(&self, pledge: &Pledge) -> Result<Pledge, PledgeRepositoryError> {
        sqlx::query("INSERT INTO pledges (id, name, description) VALUES (?, ?, ?)")
            .bind(pledge.id)
            .bind(&pledge.name)
            .bind(&pledge.description)
            .execute(&*self.pool)
            .await?;

        Ok(pledge.clone())
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<Pledge, PledgeRepositoryError> {
        let row = sqlx::query("SELECT id, name, description FROM pledges WHERE id = ?")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        match row {
            Some(row) => Ok(Self::row_to_pledge(row)),
            None => Err(PledgeRepositoryError::NotFound),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Pledge>, PledgeRepositoryError> {
        let rows = sqlx::query("SELECT id, name, description FROM pledges")
            .fetch_all(&*self.pool)
            .await?;

        Ok(rows.into_iter().map(Self::row_to_pledge).collect())
    }

    pub async fn update(&self, pledge: &Pledge) -> Result<Pledge, PledgeRepositoryError> {
        let result = sqlx::query("UPDATE pledges SET name = ?, description = ? WHERE id = ?")
            .bind(&pledge.name)
            .bind(&pledge.description)
            .bind(pledge.id)
            .execute(&*self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(PledgeRepositoryError::NotFound);
        }

        Ok(pledge.clone())
    }

    pub async fn delete(&self, id: &Uuid) -> Result<bool, PledgeRepositoryError> {
        let result = sqlx::query("DELETE FROM pledges WHERE id = ?")
            .bind(id)
            .execute(&*self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    fn row_to_pledge(row: PgRow) -> Pledge {
        Pledge {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
        }
    }
}
