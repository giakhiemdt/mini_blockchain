use std::sync::Arc;

use axum::response::IntoResponse;
use axum::Extension;
use axum::{Json, extract::State};
use axum::http::StatusCode;
use crate::middlewares::auth::AppState;
use crate::models::jwt::Claims;
use crate::models::response_basic::ResponseModel; 
use crate::models::user::UserInformation;
use crate::db::repo::user_repo;

pub async fn get_me(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {

    let pool = &state.db;

    match  user_repo::find_user_by_id(pool, &claims.sub).await {
        Ok(user) => {
            let response = UserInformation {
                name: user.name,
                email: user.email,
                profile_pic_url: user.profile_pic_url,
                created_at: user.created_at,
                updated_at: user.updated_at,
                is_verified: user.is_verified,
            };

            Ok((StatusCode::OK, 
                Json(ResponseModel::<UserInformation> {
                    is_success: true,
                    result: Some(response),
                    message: "Successful!".to_string(),
                })
            ))
        },
        Err(_) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseModel::<()> {
                    is_success: false,
                    result: None,
                    message: "Invalid user id!".to_string(),
                })
            ))
        }
    }

}