use axum::{Router, routing::post, middleware};
use std::sync::Arc;

use crate::middlewares::auth::{auth_middleware, AppState};
use crate::services::transaction_service::create_transaction;

pub fn protected_transaction_routes(state:Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/transaction/create", post(create_transaction))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
}
