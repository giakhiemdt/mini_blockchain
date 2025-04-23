use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegisterRequest {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub name: String,
    pub token: String,
}