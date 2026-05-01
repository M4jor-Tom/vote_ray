use axum::{routing::get, Router};
use std::sync::Arc;

use crate::database::{
    candidate_repository::CandidateRepository, pledge_repository::PledgeRepository,
};
use crate::rest::handlers::candidate::{
    create_candidate, delete_candidate, get_all_candidates, get_candidate, update_candidate,
    CandidateHandler,
};
use crate::rest::handlers::pledge::{
    create_pledge, delete_pledge, get_all_pledges, get_pledge, update_pledge, PledgeHandler,
};

pub fn create_v1_router(
    candidate_repository: Arc<CandidateRepository>,
    pledge_repository: Arc<PledgeRepository>,
) -> Router {
    let candidate_handler = Arc::new(CandidateHandler::new(candidate_repository));
    let pledge_handler = Arc::new(PledgeHandler::new(pledge_repository));

    Router::new()
        .route(
            "/candidates",
            get(get_all_candidates).post(create_candidate),
        )
        .route(
            "/candidates/:id",
            get(get_candidate)
                .put(update_candidate)
                .delete(delete_candidate),
        )
        .with_state(candidate_handler)
        .route("/pledges", get(get_all_pledges).post(create_pledge))
        .route(
            "/pledges/:id",
            get(get_pledge).put(update_pledge).delete(delete_pledge),
        )
        .with_state(pledge_handler)
}
