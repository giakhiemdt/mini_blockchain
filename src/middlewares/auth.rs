use axum::{
    extract::State, http::StatusCode, middleware::Next, response::Response, body::Body
};
use std::sync::Arc;
use axum::http::Request;
use crate::utils::jwt::validate_jwt;
use sqlx::{MySql, Pool};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<MySql>,
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    
    let Some(auth_header) = req.headers().get("Authorization") else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let auth_str = auth_header.to_str().unwrap_or("");
    if !auth_str.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = auth_str.trim_start_matches("Bearer ").trim();

    let claims = validate_jwt(token, state.db.clone())
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}