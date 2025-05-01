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
use crate::db::repo::{user_repo, jwt_repo};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserRegisterRequest>,
) -> impl IntoResponse {

    let pool = &state.db;

    match user_repo::insert_new_user(pool, &payload.name, &payload.email, &payload.password).await {
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
                    message: "Register failed!".to_string(),
                })
            ))
        }
    }

}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserLoginRequest>,
) -> impl IntoResponse {

    let pool = &state.db;

    match user_repo::find_user_by_username_and_password(pool, &payload.name, &payload.password).await {
        Ok(user) => {
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
) -> impl IntoResponse {

    let pool = &state.db;

    match jwt_repo::delete_jwt(pool, &claims.sub).await {
        Ok(_) => {
            Ok((StatusCode::OK, 
                Json(ResponseModel::<()> {
                    is_success: true,
                    result: None,
                    message: "Logout successful!".to_string(),
                })
            ))
        },
        Err(_) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseModel::<()> {
                    is_success: false,
                    result: None,
                    message: "Logout failed!".to_string(),
                })
            ))
        }
    }

}