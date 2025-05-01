use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use chrono::Utc;
use secp256k1::{Secp256k1, Message, PublicKey};
use secp256k1::ecdsa::Signature;
use sha3::{Digest, Keccak256};
use hex::{FromHex, ToHex};
use tracing::info;


use crate::middlewares::auth::AppState;
use crate::models::jwt::Claims;
use crate::models::response_basic::ResponseModel;
use crate::models::transaction::CreateTransactionRequest;
use crate::db::repo::{wallet_repo, transaction_repo};

pub async fn create_transaction(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateTransactionRequest>
) -> impl IntoResponse {
    info!("NIhaoma0 {}", payload.signature);

    if payload.tx_data.amount <= BigDecimal::zero() {
        return Err((StatusCode::BAD_REQUEST,
            Json(ResponseModel::<()> {
                is_success: false,
                result: None,
                message: "Amount must be greater than 0!".to_string(),
            })
        ));
    }

    let pool = &state.db;
    let timestamp = get_current_timestamp();

    if !wallet_repo::check_wallet_owner(&state.db, &payload.tx_data.sender_wallet_address, &claims.sub).await {
        return Err((
            StatusCode::FORBIDDEN, 
            Json(ResponseModel::<()> {
                is_success: false,
                result: None,
                message: "You are not the owner of this wallet!".to_string(),
            })
        ));
    }

    let sender_wallet_id = wallet_repo::find_wallet_id_by_address(pool, 
        &payload.tx_data.sender_wallet_address).await;
    let receiver_wallet_id = wallet_repo::find_wallet_id_by_address(pool, 
        &payload.tx_data.receiver_wallet_address).await;

    let public_key = wallet_repo::find_wallet_public_key_by_address(pool, 
        &payload.tx_data.sender_wallet_address).await;

    let next_nonce = match transaction_repo::get_current_nonce(pool, &sender_wallet_id).await {
        Ok(nonce) => nonce + 1,
        Err(err) => {
            info!("Failed to get nonce: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseModel::<()> {
                    is_success: false,
                    result: None,
                    message: "Failed to get nonce!".to_string(),
                })
            ));
        }
    };
        
        

    let tx_data = format!(
        "{}{}{}{}", 
        payload.tx_data.sender_wallet_address, 
        payload.tx_data.receiver_wallet_address, 
        payload.tx_data.amount, 
        payload.tx_data.nonce
    );

    let (is_valid, tx_hash) = verify_signature(&tx_data, &payload.signature, &public_key);
        
    if is_valid {
        match transaction_repo::insert_new_transaction(
            pool, &sender_wallet_id, &receiver_wallet_id, 
            payload.tx_data.amount, &timestamp, &payload.signature, 
            &tx_hash, BigDecimal::from_f64(0.001).unwrap(), next_nonce
        ).await {
            Ok(_) => {
                Ok((StatusCode::OK,
                    Json(ResponseModel::<()> {
                        is_success: true,
                        result: None,
                        message: "Transaction created successfully!".to_string(),
                    })
                ))
            },
            Err(err) => {
                eprintln!("Error creating transaction: {:?}", err);
                Err((StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseModel::<()> {
                        is_success: false,
                        result: None,
                        message: "Failed to create transaction!".to_string(),
                    })
                ))
            }
        }
    } else {
        Err((StatusCode::BAD_REQUEST,
            Json(ResponseModel::<()> {
                is_success: false,
                result: None,
                message: "Invalid signature!".to_string(),
            })
        ))
    }
}

pub fn verify_signature(
    tx_data: &str,
    signature_hex: &str,
    public_key_hex: &str
) -> (bool, String) {
    info!("public_key_hex: {}", public_key_hex);
    info!("signature_hex: {}", signature_hex);
    info!("tx_data: {}", tx_data);


    // 1. Băm tx_data -> lấy tx_hash
    let mut hasher = Keccak256::new();
    hasher.update(tx_data.as_bytes());
    let tx_hash_bytes = hasher.finalize();
    
    // 2. Chuyển Keccak256 output thành [u8; 32]
    let tx_hash: [u8; 32] = tx_hash_bytes.into();
    
    // 3. Decode public_key và signature từ hex
    info!("public_key_hex length: {}", public_key_hex.len());
    
    // Kiểm tra và xử lý public key có 65 bytes (nếu có thêm 0x04)
    let mut public_key_bytes = Vec::<u8>::from_hex(public_key_hex).unwrap();
    
    // Nếu public_key không có prefix 0x04, thêm vào
    if public_key_bytes.len() == 64 {
        let mut full_key = vec![0x04]; // Uncompressed key bắt đầu với 0x04
        full_key.extend(public_key_bytes);
        public_key_bytes = full_key; // Gán lại public key đầy đủ
    }
    
    info!("public_key_bytes length: {}", public_key_bytes.len());
    
    // Kiểm tra độ dài của public_key_bytes
    assert_eq!(public_key_bytes.len(), 65, "Public key phải có 65 bytes");
    
    let public_key = PublicKey::from_slice(&public_key_bytes).unwrap();
    
    // 4. Decode signature từ hex
    let signature_bytes = Vec::<u8>::from_hex(signature_hex).unwrap();
    let signature = Signature::from_compact(&signature_bytes).unwrap();
    
    info!("Signature decoded");
    
    // 5. Verify signature
    let secp = Secp256k1::new();
    let message = Message::from_digest(tx_hash);
    let is_valid = secp.verify_ecdsa(message, &signature, &public_key).is_ok();
    
    // 6. Encode tx_hash thành hex để trả về
    let tx_hash_hex = tx_hash.encode_hex::<String>();
    
    (is_valid, tx_hash_hex)
}




fn get_current_timestamp() -> String {
    let timestamp = Utc::now();
    timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
}

