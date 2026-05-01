use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

use crate::business::candidate::Candidate;

#[derive(Debug, Clone)]
pub struct CandidateRepository {
    pool: Arc<PgPool>,
}

#[derive(Debug, thiserror::Error)]
pub enum CandidateRepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Candidate not found")]
    NotFound,
}

impl CandidateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    pub async fn create(&self, candidate: &Candidate) -> Result<Candidate, CandidateRepositoryError> {
        sqlx::query("INSERT INTO candidates (id, name) VALUES (?, ?)")
            .bind(candidate.id)
            .bind(&candidate.name)
            .execute(&*self.pool)
            .await?;

        Ok(candidate.clone())
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<Candidate, CandidateRepositoryError> {
        let row = sqlx::query("SELECT id, name FROM candidates WHERE id = ?")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        match row {
            Some(row) => Ok(Self::row_to_candidate(row)),
            None => Err(CandidateRepositoryError::NotFound),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Candidate>, CandidateRepositoryError> {
        let rows = sqlx::query("SELECT id, name FROM candidates")
            .fetch_all(&*self.pool)
            .await?;

        Ok(rows.into_iter().map(Self::row_to_candidate).collect())
    }

    pub async fn update(&self, candidate: &Candidate) -> Result<Candidate, CandidateRepositoryError> {
        let result = sqlx::query("UPDATE candidates SET name = ? WHERE id = ?")
            .bind(&candidate.name)
            .bind(candidate.id)
            .execute(&*self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(CandidateRepositoryError::NotFound);
        }

        Ok(candidate.clone())
    }

    pub async fn delete(&self, id: &Uuid) -> Result<bool, CandidateRepositoryError> {
        let result = sqlx::query("DELETE FROM candidates WHERE id = ?")
            .bind(id)
            .execute(&*self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    fn row_to_candidate(row: PgRow) -> Candidate {
        Candidate {
            id: row.get("id"),
            name: row.get("name"),
        }
    }
}
