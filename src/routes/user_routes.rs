use axum::{Router, routing::get, middleware};
use std::sync::Arc;

use crate::middlewares::auth::{auth_middleware, AppState};
use crate::services::user_service::get_me;

pub fn protected_user_routes(state:Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/account/me", get(get_me))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
}