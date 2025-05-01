use axum::routing::get;
use axum::{Router, routing::post, middleware};
use std::sync::Arc;

use crate::middlewares::auth::{auth_middleware, AppState};
use crate::services::wallet_service::{create_wallet, get_wallet_information};

pub fn protected_wallet_routes(state:Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/wallet/create", post(create_wallet))
        .route("/wallet/information", get(get_wallet_information))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
}