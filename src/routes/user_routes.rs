use axum::{Router, routing::{get, put}, middleware};
use std::sync::Arc;

use crate::middlewares::auth::{auth_middleware, AppState};
use crate::services::user_service::{get_me, update_information};

pub fn protected_user_routes(state:Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/account/me", get(get_me))
        .route("/account/me", put(update_information))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
}