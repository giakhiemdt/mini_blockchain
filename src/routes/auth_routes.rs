use axum::{Router, routing::{post, delete}, middleware};
use std::sync::Arc;
use crate::services::auth_service::{register, login, logout};

use crate::middlewares::auth::{auth_middleware, AppState};

pub fn public_auth_routes() -> Router<Arc<AppState>> { // public
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

pub fn protected_auth_routes(state:Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/logout", delete(logout))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
}