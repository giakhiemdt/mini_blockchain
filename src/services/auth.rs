use std::sync::Arc;

use axum::response::IntoResponse;
use axum::Extension;
use axum::{Json, extract::State};
use axum::http::StatusCode;
use crate::middlewares::auth::AppState;
use crate::models::jwt::Claims;
use crate::models::auth::{
    UserLoginRequest, UserRegisterRequest,
    UserLoginResponse
};
use crate::models::response_basic::ResponseModel; 
use crate::utils::jwt::create_jwt;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserRegisterRequest>,
) -> Result<(StatusCode, impl IntoResponse), (StatusCode, impl IntoResponse)> {

    let pool = &state.db;

    let result = sqlx::query!(
        r#"
        INSERT INTO Users (name, email, password)
        VALUES (?, ?, ?)
        "#,
        payload.name,
        payload.email,
        payload.password,
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            Ok((StatusCode::OK,
                Json(ResponseModel::<()> {
                    is_success: true,
                    result: None,
                    message: "Register successful!".to_string(),
                })
            ))
        },
        Err(err) => {
            eprintln!("Lỗi khi tạo user: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseModel::<()> {
                    is_success: false,
                    result: None,
                    message: "Không thể tạo user".to_string(),
                })
            ))
        }
    }

}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserLoginRequest>,
) -> Result<(StatusCode, impl IntoResponse), (StatusCode, impl IntoResponse)> {

    let pool = &state.db;

    let user = sqlx::query!(
        r#"
        SELECT id, name, email, role FROM Users WHERE name = ? AND password = ?
        "#,
        payload.name,
        payload.password,
    )
    .fetch_optional(pool) 
    .await;

    match user {
        Ok(Some(user)) => {

            let response = UserLoginResponse {
                name: user.name,
                token: create_jwt(
                    &user.id.to_string(), 
                    &user.email.to_string(),
                    user.role.as_ref().unwrap().as_str(),
                    pool.clone()
                ).await,
            };

            Ok((StatusCode::OK,
                Json(ResponseModel::<UserLoginResponse> {
                    is_success: true,
                    result: Some(response), 
                    message: "Login successful!".to_string(),
                })))
        },
        Ok(None) => {
            Err((StatusCode::UNAUTHORIZED, 
                Json(ResponseModel::<()> {
                    is_success: false,
                    result: None,
                    message: "Invalid credentials".to_string(),
                })))
        },
        Err(err) => {
            eprintln!("Error while fetching user: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, 
                Json(ResponseModel::<()> {
                    is_success: false,
                    result: None,
                    message: "Internal server error".to_string(),
                })))
        }
    }
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<(StatusCode, impl IntoResponse), (StatusCode, impl IntoResponse)> {

    let pool = &state.db;

    let result = sqlx::query!(
        r#"
        DELETE FROM Jwts WHERE user_id = ?
        "#,
        claims.sub
    )
    .fetch_optional(pool) 
    .await;

    match result {
        Ok(_) => {
            Ok((StatusCode::OK, 
                Json(ResponseModel::<()> {
                    is_success: true,
                    result: None,
                    message: "Đăng xuất thành công!".to_string(),
                })
            ))
        },
        Err(_) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseModel::<()> {
                    is_success: false,
                    result: None,
                    message: "Lỗi khi đăng xuất.".to_string(),
                })
            ))
        }
    }

}