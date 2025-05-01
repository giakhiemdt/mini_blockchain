mod routes;
mod db;
mod services;
mod models;
mod utils;
mod middlewares;

use axum::Router;
use middlewares::auth::AppState;
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;
use std::sync::Arc;
use tracing_subscriber;

use routes::{
    protected_auth_routes, protected_user_routes, protected_wallet_routes, public_auth_routes, transaction_routes::protected_transaction_routes
};
use db::database::connect_db;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let pool = connect_db().await;

    let state = Arc::new(
        AppState { 
            db: pool
        }
    );

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);

    let app = Router::new()
        .merge(public_auth_routes())
        .merge(protected_auth_routes(state.clone()))
        .merge(protected_user_routes(state.clone()))
        .merge(protected_wallet_routes(state.clone()))
        .merge(protected_transaction_routes(state.clone()))
        .layer(cors)
        .with_state(state.clone());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
