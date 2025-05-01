use std::sync::Arc;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::State, Extension};
use sha2::{Sha256, Digest}; 
use hex::encode;  
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;
use rand::TryRngCore;
use hex;

use crate::middlewares::auth::AppState;
use crate::models::jwt::Claims;
use crate::models::response_basic::ResponseModel;
use crate::models::wallet::{CreateWalletRequest, GetWalletInformationRequest, GetWalletInformationResponse};
use crate::db::repo::wallet_repo;

pub async fn create_wallet(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateWalletRequest>
) -> impl IntoResponse {

    let pool = &state.db;

    let (private_key, public_key) = generate_keypair();
    let address= create_wallet_address(
        claims.sub.clone(), public_key.clone());

    match wallet_repo::insert_new_wallet(pool, &claims.sub, &payload.wallet_name, &address, &public_key).await {
        Ok(_) => {
            Ok((StatusCode::OK,
                Json(ResponseModel::<String> {
                    is_success: true,
                    result: Some(private_key),
                    message: "Create wallet successful!".to_string(),
                })
            ))
        },
        Err(err) => {
            eprintln!("Lỗi khi tạo user: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseModel::<()> {
                    is_success: false,
                    result: None,
                    message: "Create wallet failed!".to_string(),
                })
            ))
        }
    }

}

pub async fn get_wallet_information(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<GetWalletInformationRequest>
) -> impl IntoResponse {

    let pool = &state.db;

    match wallet_repo::find_wallet_by_id_and_userid(pool, &payload.wallet_id, &claims.sub).await {
        Ok(result) => {
            let wallet_info = GetWalletInformationResponse {
                name: result.name,
                balance: result.balance,
                created_at: result.created_at,
                address: result.address,
                public_key: result.public_key
            };
        
            Ok((StatusCode::OK,
                Json(ResponseModel::<GetWalletInformationResponse> {
                    is_success: true,
                    result: Some(wallet_info), 
                    message: "Get wallet information successful!".to_string(),
                })
            ))
        }
        Err(err) => {
            eprintln!("Lỗi khi tạo user: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseModel::<()> {
                    is_success: false,
                    result: None,
                    message: "Get wallet information failed!".to_string(),
                })
            ))
        }
    }


}

fn create_wallet_address(user_id: String, public_key: String) -> String {
    
    let input = format!("{}-{}", user_id, public_key);
    
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    
    encode(result)
}



pub fn generate_keypair() -> (String, String) {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    
    let mut secret_key_data = [0u8; 32];
    rng.try_fill_bytes(&mut secret_key_data).expect("Failed to generate random bytes");
    
    let secret_key = SecretKey::from_slice(&secret_key_data)
        .expect("Generated private key is invalid");

    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    
    let private_key_hex = hex::encode(secret_key.secret_bytes());

    let serialized_pubkey = public_key.serialize_uncompressed();
    let public_key_hex = hex::encode(&serialized_pubkey[1..]);
    
    (private_key_hex, public_key_hex)
}




